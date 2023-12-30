mod bitset;
mod particle;
mod performance;
mod render;
mod utils;

use particle::{Operation, Particle};
use wasm_bindgen::prelude::*;
use web_sys::js_sys::Math;

use crate::render::{ctx2d::Ctx2d, Context};
use performance::Performance;

const MAX_PARTICLES: usize = 1 << 13;

#[wasm_bindgen]
pub struct Fireworks {
    width: f32,
    height: f32,
    ctx: Ctx2d,
    last_time: f64,
    render_throttle: utils::Throttle,
    perf: Performance,

    particles: Vec<Particle>,
    new_particles: Vec<Particle>,
}

#[wasm_bindgen]
impl Fireworks {
    #[wasm_bindgen]
    pub fn new() -> Result<Fireworks, JsValue> {
        utils::set_panic_hook();

        let window = web_sys::window().unwrap();
        let width = window.inner_width()?.as_f64().unwrap() as f32;
        let height = window.inner_height()?.as_f64().unwrap() as f32;

        let ctx = Ctx2d::new(width, height)?;
        // let ctx = Pixi{};

        let last_time = window.performance().unwrap().now();

        let render_throttle = utils::Throttle::new(1.0 / 30.0);

        let perf = Performance::new();

        let particles = Vec::with_capacity(MAX_PARTICLES);
        let new_particles = Vec::new();

        Ok(Self {
            width,
            height,
            ctx,
            last_time,
            perf,
            render_throttle,
            particles,
            new_particles,
        })
    }

    #[wasm_bindgen]
    pub fn tick(&mut self) {
        let dt = self.calc_dt();
        self.perf.fps.update(1.0 / dt);
        if dt < 0.2 {
            self.perf.update.start();
            self.update(dt);
            self.perf.update.end();

            if self.render_throttle.get(dt) {
                self.perf.render.start();
                self.render();
                self.perf.render.end();
            }
        }
    }

    fn calc_dt(&mut self) -> f32 {
        let now = web_sys::window().unwrap().performance().unwrap().now();
        let dt = (now - self.last_time) / 1000.0; // in seconds
        self.last_time = now;
        dt as f32
    }

    pub fn update(&mut self, dt: f32) {
        if self.particles.len() >= (MAX_PARTICLES as f64 * 0.9) as usize {
            let message = format!("particles almost at max size: {}", self.particles.len());
            web_sys::console::log_1(&message.into());
        }

        // Update existing particles
        for i in (0..self.particles.len()).rev() {
            let result =
                unsafe { self.particles.get_unchecked_mut(i) }.update(dt, self.width, self.height);

            let mut already_dead = false;
            for op in result {
                match op {
                    Operation::Die => {
                        if !already_dead {
                            self.particles.swap_remove(i);
                            already_dead = true;
                        }
                    }
                    Operation::Push(p) => {
                        self.new_particles.push(p);
                    }
                }
            }
        }

        // Process new particles
        for p in self.new_particles.drain(..) {
            if self.particles.len() < MAX_PARTICLES {
                self.particles.push(p);
            }
        }

        // Randomly generate new particles
        if Math::random() < 0.1 && self.particles.len() < MAX_PARTICLES {
            self.particles
                .push(Particle::random(self.width, self.height));
        }
    }

    pub fn render(&mut self) {
        self.ctx.clear(self.width, self.height);
        for p in &self.particles {
            p.render(&self.ctx);
        }
    }
}

#[wasm_bindgen]
pub fn ping() {
    web_sys::console::log_1(&"[WASM] ping received".into());
}
