mod bitset;
mod font;
mod particle;
mod performance;
mod render;
mod utils;

use particle::{Operation, Particle};
use wasm_bindgen::prelude::*;
use web_sys::js_sys::Math;

use crate::{
    render::{ctx2d::Ctx2d, Context},
    utils::{now, rand32},
};
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

    /// In seconds. Not a duration because I don't know how negative durations behave
    countdown: i64,

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

        let last_time = now();

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
            countdown: 0,
            particles,
            new_particles,
        })
    }

    #[wasm_bindgen]
    pub fn tick(&mut self) {
        let dt = self.calc_dt();
        self.perf.fps.update(1.0 / dt as f64);
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

    #[wasm_bindgen]
    pub fn push_lerper(
        &mut self,
        x1: f32,
        y1: f32,
        explosion_particles: Option<usize>,
        duration: f32,
    ) {
        let x0 = rand32(x1 - 200.0, x1 + 200.0).max(0.0).min(self.width);
        let y0 = self.height - 1.0;
        self.new_particles.push(Particle::lerper(
            x0,
            y0,
            x1,
            y1,
            explosion_particles,
            duration,
        ));
    }

    fn calc_dt(&mut self) -> f32 {
        let n = now();
        let dt = (n - self.last_time) / 1000.0; // in seconds
        self.last_time = n;
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
        let dice = Math::random() < 0.08;
        let can_push = self.particles.len() < MAX_PARTICLES;
        let is_new_year = self.countdown <= 0;
        if is_new_year && dice && can_push {
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

    #[wasm_bindgen]
    pub fn update_countdown(&mut self, seconds: f64) {
        let seconds = (seconds as i64).max(0);

        if seconds == self.countdown {
            return;
        }

        self.countdown = seconds;

        let s = seconds % 60;
        let m = (seconds / 60) % 60;
        let h = seconds / 60 / 60;

        let mut text = font::Text::new();
        if seconds == 0 {
            text.push(2).push(0).push(2).push(4);
        } else if seconds < 10 {
            text.push(s);
        } else {
            if h > 0 {
                text.push(h / 10).push(h % 10).push(10); // :
            }
            if m > 0 || h > 0 {
                text.push(m / 10).push(m % 10).push(10); // :
            }
            if s > 0 || m > 0 || h > 0 {
                text.push(s / 10).push(s % 10);
            }
        }

        let points = text
            .scale(128.)
            .center(font::Point::new(self.width / 2., self.height / 2.))
            .build();

        for p in points {
            self.push_lerper(p.x, p.y, Some(1), 1.0);
        }
    }
}

#[wasm_bindgen]
pub fn ping() {
    web_sys::console::log_1(&"[WASM] ping received".into());
}
