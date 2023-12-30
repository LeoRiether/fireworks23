use wasm_bindgen::{JsCast, JsValue};

use super::Context;

pub struct Ctx2d {
    ctx: web_sys::CanvasRenderingContext2d,
}

impl Ctx2d {
    pub fn new(width: f32, height: f32) -> Result<Self, JsValue> {
        let canvas = Self::create_canvas(width, height)?;

        let window = web_sys::window().unwrap();
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

        Ok(Self { ctx })
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
}

impl Context for Ctx2d {
    fn clear(&self, width: f32, height: f32) {
        self.ctx.set_fill_style(&"#000".into());
        self.ctx.fill_rect(0.0, 0.0, width as f64, height as f64);
    }

    fn circle(&self, x: f32, y: f32, r: f32, color: &str, alpha: f32) {
        self.ctx.begin_path();
        self.ctx.set_fill_style(&color.into());
        self.ctx.set_global_alpha(alpha as f64);
        self.ctx
            .arc(
                x as f64,
                y as f64,
                r as f64,
                0.0,
                2.0 * std::f64::consts::PI,
            )
            .unwrap();
        self.ctx.fill();
    }
}
