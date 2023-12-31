use crate::{
    render::Context,
    utils::{qlerp, rand32, random_color},
};
use std::{mem::take, vec};

const GRAVITY: f32 = 500.0;

#[derive(Clone)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub color: String,
    pub alpha: f32,
    pub behaviors: Vec<Behavior>,
}

#[derive(Clone)]
pub enum Behavior {
    Rigidbody,
    Lerper(Box<Lerper>),
    Slowdown(f32),
    HasFuse(f32),
    DoubleFuse(f32),
    Fades { alpha: f32, factor: f32 },
    Sparkles { time: f32, cycle_duration: f32 },
    DiesAfter(f32),
    LeavesTrail,
    LeavesSparklingTrail,
}

#[derive(Clone)]
struct Lerper {
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    t: f32,
    duration: f32,
}

pub enum Operation {
    Push(Particle),
    Die,
}

impl Particle {
    pub fn random(width: f32, height: f32) -> Self {
        let mut behaviors = vec![Behavior::Rigidbody];

        if rand32(0.0, 1.0) < 0.5 {
            behaviors.push(Behavior::LeavesTrail);
        } else {
            behaviors.push(Behavior::LeavesSparklingTrail);
        }

        let fuse = rand32(0.5, 1.5);
        if rand32(0.0, 1.0) < 0.1 {
            behaviors.push(Behavior::DoubleFuse(fuse));
        } else {
            behaviors.push(Behavior::HasFuse(fuse));
        };

        Self {
            x: rand32(0.0, width),
            y: height - 1.0,
            vx: rand32(-200.0, 200.0),
            vy: rand32(-1100.0, -500.0),
            color: random_color(),
            alpha: 1.0,
            behaviors,
        }
    }

    pub fn lerper(x0: f32, y0: f32, x1: f32, y1: f32, duration: f32) -> Self {
        let mut behaviors = vec![Behavior::Lerper(Box::new(Lerper {
            x0,
            y0,
            x1,
            y1,
            t: 0.0,
            duration,
        }))];

        if rand32(0.0, 1.0) < 0.5 {
            behaviors.push(Behavior::LeavesTrail);
        } else {
            behaviors.push(Behavior::LeavesSparklingTrail);
        }

        Self {
            x: x0,
            y: y0,
            vx: 0.0,
            vy: 0.0,
            color: random_color(),
            alpha: 1.0,
            behaviors,
        }
    }

    #[inline]
    pub fn update(&mut self, dt: f32, width: f32, height: f32) -> Vec<Operation> {
        let mut result = Vec::new();

        self.alpha = 1.0;

        // Update behaviors
        let mut behaviors = take(&mut self.behaviors);
        for behavior in &mut behaviors {
            behavior.update(self, dt, &mut result);
        }
        self.behaviors = behaviors;

        // Check if particle is out of bounds
        if self.x < 0.0 || self.x > width || self.y > height {
            result.push(Operation::Die);
        }

        result
    }

    #[inline]
    pub fn render<C: Context>(&self, ctx: &C) {
        ctx.circle(self.x, self.y, 1.3, &self.color, self.alpha);
    }
}

impl Behavior {
    #[inline]
    pub fn update(&mut self, p: &mut Particle, dt: f32, result: &mut Vec<Operation>) {
        use Behavior::*;
        match self {
            Rigidbody => {
                p.x += p.vx * dt;
                p.y += p.vy * dt + 0.5 * GRAVITY * dt * dt;
                p.vy += GRAVITY * dt;
            }
            Slowdown(factor) => {
                p.vy -= 0.98 * GRAVITY * dt; // gravity does not affect as much I guess??
                p.vx *= *factor;
                p.vy *= *factor;
            }
            Lerper(lerper) => {
                lerper.t += dt;
                if lerper.t >= lerper.duration {
                    result.push(Operation::Die);
                    let mut child = p.clone();
                    child.behaviors = random_explosion_behaviors();
                    let n = rand32(80.0, 160.0) as usize;
                    explode(n, child, result);
                } else {
                    let t = lerper.t / lerper.duration;
                    p.x = qlerp(lerper.x0, lerper.x1, t);
                    p.y = qlerp(lerper.y0, lerper.y1, t);
                }
            }
            HasFuse(fuse) => {
                *fuse -= dt;
                if *fuse <= 0.0 {
                    result.push(Operation::Die);
                    let mut child = p.clone();
                    child.behaviors = random_explosion_behaviors();
                    let n = rand32(80.0, 160.0) as usize;
                    explode(n, child, result);
                }
            }
            DoubleFuse(fuse) => {
                *fuse -= dt;
                if *fuse <= 0.0 {
                    result.push(Operation::Die);
                    let mut child = p.clone();
                    child.behaviors = vec![
                        Behavior::Rigidbody,
                        Behavior::Slowdown(0.95),
                        Behavior::HasFuse(rand32(0.5, 1.5)),
                    ];
                    let n = rand32(5.0, 15.0) as usize;
                    explode(n, child, result);
                }
            }
            Fades { alpha, factor } => {
                *alpha *= if *alpha > 0.6 { (*factor * 1.03).min(0.99) } else { *factor };
                if *alpha < 0.15 {
                    result.push(Operation::Die);
                }
                p.alpha *= *alpha;
            }
            Sparkles {
                time,
                cycle_duration,
            } => {
                *time += dt;

                let alpha =
                    0.5 + 0.5 * (2.0 * std::f32::consts::PI * *time / *cycle_duration).cos();
                p.alpha *= alpha;
            }
            DiesAfter(time) => {
                *time -= dt;
                if *time <= 0.0 {
                    result.push(Operation::Die);
                }
            }
            LeavesTrail => {
                let mut trail = p.clone();
                trail.behaviors = vec![Behavior::Fades {
                    alpha: 1.0,
                    factor: 0.94,
                }];
                trail.vx *= 0.1;
                trail.vy *= 0.1;
                result.push(Operation::Push(trail));
            }
            LeavesSparklingTrail => {
                let mut trail = p.clone();
                trail.behaviors = vec![
                    Behavior::Fades {
                        alpha: 1.0,
                        factor: 0.94,
                    },
                    Behavior::Sparkles {
                        time: rand32(0.0, 0.8),
                        cycle_duration: rand32(0.1, 0.8),
                    },
                ];
                trail.vx *= 0.1;
                trail.vy *= 0.1;
                result.push(Operation::Push(trail));
            }
        }
    }
}

fn explode(n: usize, mut p: Particle, result: &mut Vec<Operation>) {
    p.vx *= 0.05;
    p.vy *= 0.05;
    for _ in 0..n {
        let mut child = p.clone();
        modify_child_behaviors(&mut child.behaviors);

        let (vx, vy) = random_velocity_in_sphere(55000000.0);
        child.vx += vx;
        child.vy += vy;
        result.push(Operation::Push(child));
    }
}

fn random_velocity_in_sphere(radius: f32) -> (f32, f32) {
    let alpha = rand32(0.0, 2.0 * std::f32::consts::PI);
    let mag = rand32(0.0, radius).cbrt();
    (mag * alpha.cos(), mag * alpha.sin())
}

fn random_explosion_behaviors() -> Vec<Behavior> {
    let slowdown = rand32(0.94, 0.97);
    let mut behaviors = vec![
        Behavior::Rigidbody,
        Behavior::Slowdown(slowdown),
        Behavior::Fades {
            alpha: 1.0,
            factor: rand32(0.97, 0.985),
        },
    ];

    match rand32(0.0, 1.0) {
        x if x < 0.02 => behaviors.push(Behavior::LeavesTrail),
        x if x < 0.05 => behaviors.push(Behavior::LeavesSparklingTrail),
        _ => {}
    }

    if rand32(0.0, 1.0) < 0.3 {
        behaviors.push(Behavior::Sparkles {
            time: 0.0,
            cycle_duration: rand32(0.1, 0.5),
        });
        behaviors.push(Behavior::DiesAfter(rand32(0.7, 2.0)));
    }

    behaviors
}

fn modify_child_behaviors(behaviors: &mut Vec<Behavior>) {
    for b in behaviors {
        match b {
            Behavior::Fades { alpha: _, factor } => {
                *factor *= rand32(0.94, 1.01);
            }
            Behavior::Sparkles {
                time,
                cycle_duration,
            } => {
                *time = rand32(0.0, *cycle_duration);
            }
            _ => {}
        }
    }
}
