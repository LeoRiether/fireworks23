mod bitset;
mod particle;
mod utils;

use particle::{Operation, Particle};
use wasm_bindgen::prelude::*;

const MAX_PARTICLES: usize = 4096;

#[wasm_bindgen]
pub struct Fireworks {
    width: f32,
    height: f32,
    ctx: web_sys::WebGl2RenderingContext,
    last_time: f64,

    particles: Vec<Particle>,
    new_particles: Vec<Particle>,
}

#[wasm_bindgen]
impl Fireworks {
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
            .get_context("webgl2")?
            .unwrap()
            .dyn_into::<web_sys::WebGl2RenderingContext>()
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
        canvas.set_attribute("overflow", "hidden")?;
        Ok(canvas)
    }

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
        // Update existing particles
        for i in (0..self.particles.len()).rev() {
            let result = unsafe { self.particles.get_unchecked_mut(i) }.update(dt);
            for op in result {
                match op {
                    Operation::Die => {
                        self.particles.swap_remove(i);
                    }
                    Operation::Push(p) => {
                        self.new_particles.push(p);
                    }
                }
            }
        }

        // Process new particles
        for p in self.new_particles.drain(..) {
            self.particles.push(p);
        }
    }

    pub fn render(&self) {

    }
}
