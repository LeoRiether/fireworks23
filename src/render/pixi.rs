use super::Context;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn renderClear();

    #[wasm_bindgen(js_namespace = window)]
    fn renderCircle(x: f64, y: f64, r: f64, color: &str, alpha: f64);
}

pub struct Pixi {}

impl Context for Pixi {
    fn clear(&self, _: f32, _: f32) {
        renderClear();
    }

    fn circle(&self, x: f32, y: f32, r: f32, color: &str, alpha: f32) {
        renderCircle(x as f64, y as f64, r as f64, color, alpha as f64);
    }
}
