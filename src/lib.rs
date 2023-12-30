mod bitset;
mod particle;
mod utils;

use particle::{Operation, Particle};
use wasm_bindgen::prelude::*;
use web_sys::js_sys::Math;

const MAX_PARTICLES: usize = 1 << 13;

#[wasm_bindgen]
pub struct Fireworks {
    width: f32,
    height: f32,
    ctx: web_sys::CanvasRenderingContext2d,
    last_time: f64,

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

        let canvas = Self::create_canvas(width, height)?;
        window
            .document()
            .unwrap()
            .body()
            .unwrap()
            .append_child(&canvas)?;

        let ctx = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let last_time = window.performance().unwrap().now();

        let particles = Vec::with_capacity(MAX_PARTICLES);
        let new_particles = Vec::new();

        Ok(Self {
            width,
            height,
            ctx,
            last_time,
            particles,
            new_particles,
        })
    }

    fn create_canvas(width: f32, height: f32) -> Result<web_sys::HtmlCanvasElement, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.create_element("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
        canvas.set_attribute("id", "world").unwrap();
        canvas.set_attribute("width", width.to_string().as_str())?;
        canvas.set_attribute("height", height.to_string().as_str())?;
        canvas.set_attribute("style", "overflow: hidden;")?;
        Ok(canvas)
    }

    #[wasm_bindgen]
    pub fn tick(&mut self) {
        let dt = self.calc_dt();
        if dt < 0.2 {
            self.update(dt);
            self.render();
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
            let message = format!("particles almost as max size: {}", self.particles.len());
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

    pub fn render(&self) {
        self.ctx.set_fill_style(&"#000".into());
        self.ctx
            .fill_rect(0.0, 0.0, self.width as f64, self.height as f64);
        for p in &self.particles {
            p.render(&self.ctx);
        }
    }
}

#[wasm_bindgen]
pub fn ping() {
    web_sys::console::log_1(&"[WASM] ping received".into());
}
