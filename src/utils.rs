use web_sys::js_sys::Math;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn rand32(min: f32, max: f32) -> f32 {
    Math::random() as f32 * (max - min) + min
}

pub struct Throttle {
    pub time: f32,
    pub interval: f32,
}

impl Throttle {
    pub fn new(interval: f32) -> Self {
        Self {
            time: 0.0,
            interval,
        }
    }

    pub fn get(&mut self, dt: f32) -> bool {
        self.time += dt;
        if self.time >= self.interval {
            self.time -= self.interval;
            true
        } else {
            false
        }
    }
}

pub fn lerp(from: f32, to: f32, p: f32) -> f32 {
    from + (to - from) * p
}

pub fn qlerp(from: f32, to: f32, p: f32) -> f32 {
    lerp(from, to, 1.0 - (p - 1.0) * (p - 1.0))
}

pub fn coolerp(from: f32, to: f32, p: f32) -> f32 {
    let p = p - 1.0;
    lerp(from, to, 1.0 - p * p * p * p * p * p)
}

pub fn random_color() -> String {
    format!("hsl({}, 80%, 60%)", rand32(0.0, 360.0))
}

pub fn now() -> f64 {
    web_sys::window().unwrap().performance().unwrap().now()
}
