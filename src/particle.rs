const GRAVITY: f32 = 9.81;

#[derive(Clone)]
pub struct Particle {
    pub m: f32,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub color: String,
    pub behaviors: Vec<Behavior>,
}

#[derive(Clone)]
pub enum Behavior {
    HasFuse(f32),
    DoubleFuse(f32),
    Fades,
    Sparkles { time: f32, cycle_duration: f32 },
    LeavesTrail { time: f32 },
}

pub enum Operation {
    Push(Particle),
    Die,
}

impl Particle {
    #[inline]
    pub fn update(&mut self, dt: f32) -> Vec<Operation> {
        let mut result = Vec::new();

        // Update position by velocity verlet
        self.x += self.vx * dt;
        self.y += self.vy * dt + 0.5 * GRAVITY * dt * dt;

        // Update behaviors
        for behavior in &mut self.behaviors {
            behavior.update(dt, &mut result);
        }

        result
    }
}

impl Behavior {
    #[inline]
    pub fn update(&mut self, dt: f32, result: &mut Vec<Operation>) {
        use Behavior::*;
        match self {
            HasFuse(fuse) => {
                *fuse -= dt;
                if *fuse <= 0.0 {
                    result.push(Operation::Die);
                }
            }
            DoubleFuse(_) => todo!(),
            Fades => todo!(),
            Sparkles {
                time,
                cycle_duration,
            } => todo!(),
            LeavesTrail { time } => todo!(),
        }
    }
}
