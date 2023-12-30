use crate::utils::rand32;
use std::{mem::take, vec};

const GRAVITY: f32 = 500.0;

#[derive(Clone)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub color: String,
    pub behaviors: Vec<Behavior>,
}

#[derive(Clone)]
pub enum Behavior {
    Rigidbody,
    Slowdown(f32),
    HasFuse(f32),
    DoubleFuse(f32),
    Fades { alpha: f32, factor: f32 },
    Sparkles { time: f32, cycle_duration: f32 },
    DiesAfter(f32),
    LeavesTrail,
    LeavesSparklingTrail,
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
            color: format!("hsl({}, 80%, 60%)", rand32(0.0, 360.0)),
            behaviors,
        }
    }

    pub fn with_behavior(mut self, behavior: Behavior) -> Self {
        self.behaviors.push(behavior);
        self
    }

    #[inline]
    pub fn update(&mut self, dt: f32, width: f32, height: f32) -> Vec<Operation> {
        let mut result = Vec::new();

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
    pub fn render(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        ctx.set_global_alpha(1.0);
        ctx.set_fill_style(&self.color.clone().into());

        for behavior in &self.behaviors {
            behavior.render(ctx);
        }

        ctx.begin_path();
        ctx.arc(
            self.x as f64,
            self.y as f64,
            1.5,
            0.0,
            2.0 * std::f64::consts::PI,
        )
        .unwrap();
        ctx.fill();
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
                *alpha *= *factor;
                if *alpha < 0.15 {
                    result.push(Operation::Die);
                }
            }
            Sparkles {
                time,
                cycle_duration: _,
            } => {
                *time += dt;
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
                    factor: 0.95,
                }];
                trail.vx *= 0.1;
                trail.vy *= 0.1;
                result.push(Operation::Push(trail));
            }
            LeavesSparklingTrail => {
                let mut trail = p
                    .clone()
                    .with_behavior(Behavior::Fades {
                        alpha: 1.0,
                        factor: 0.95,
                    })
                    .with_behavior(Behavior::Sparkles {
                        time: rand32(0.0, 0.8),
                        cycle_duration: rand32(0.1, 0.8),
                    });
                trail.vx *= 0.1;
                trail.vy *= 0.1;
                result.push(Operation::Push(trail));
            }
        }
    }

    pub fn render(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        use Behavior::*;
        match self {
            Fades { alpha, factor: _ } => {
                ctx.set_global_alpha(ctx.global_alpha() * *alpha as f64);
            }
            Sparkles {
                time,
                cycle_duration,
            } => {
                let alpha = 0.5 + 0.5 * (2.0 * std::f32::consts::PI * time / cycle_duration).cos();
                ctx.set_global_alpha(ctx.global_alpha() * alpha as f64);
            }
            Rigidbody | Slowdown(_) | HasFuse(_) | DoubleFuse(_) | DiesAfter(_) | LeavesTrail
            | LeavesSparklingTrail => {}
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
            Behavior::Fades { alpha, factor: _ } => {
                *alpha *= rand32(0.94, 0.97);
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
