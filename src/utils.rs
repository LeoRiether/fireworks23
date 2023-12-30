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
