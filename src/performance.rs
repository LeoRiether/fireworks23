use wasm_bindgen::JsCast;

fn now() -> f32 {
    web_sys::window().unwrap().performance().unwrap().now() as _
}

/// Tracks and displays an exponentially weighted moving average
pub struct DisplayedAverage {
    frame_start: f32,
    value: f32,
    display: web_sys::HtmlElement,
    frames: u8,
    suffix: &'static str,
}

impl DisplayedAverage {
    pub fn new(display: web_sys::HtmlElement, suffix: &'static str) -> Self {
        Self {
            frame_start: 0.0,
            value: 0.0,
            display,
            frames: 0,
            suffix,
        }
    }

    pub fn start(&mut self) {
        self.frame_start = now();
    }

    pub fn end(&mut self) {
        self.update(now() - self.frame_start);
    }

    pub fn update(&mut self, value: f32) {
        self.value = 0.9 * self.value + 0.1 * value;
        if !self.value.is_finite() {
            self.value = 0.0;
        }

        self.frames += 1;
        if self.frames == 10 {
            self.frames = 0;
            self.display
                .set_inner_text(&format!("{:.2}{}", self.value, self.suffix));
        }
    }
}

/// Tracks and displays performance statistics
pub struct Performance {
    pub fps: DisplayedAverage,
    pub update: DisplayedAverage,
    pub render: DisplayedAverage,
}

impl Performance {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let fps_el = document
            .get_element_by_id("fps")
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();

        let update_el = document
            .get_element_by_id("update-time")
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();

        let render_el = document
            .get_element_by_id("render-time")
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();

        Self {
            fps: DisplayedAverage::new(fps_el, " FPS"),
            update: DisplayedAverage::new(update_el, "ms update"),
            render: DisplayedAverage::new(render_el, "ms render"),
        }
    }
}
