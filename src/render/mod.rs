pub mod ctx2d;
pub mod pixi;

pub trait Context {
    fn clear(&self, width: f32, height: f32);
    fn circle(&self, x: f32, y: f32, r: f32, color: &str, alpha: f32);
}
