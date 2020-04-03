use nalgebra::Vector3;
use std::sync::Arc;

pub trait Texture: Sync + Send {
    fn value(&self, u:f32, v:f32, p: Vector3<f32>) -> Vector3<f32>;
}

pub struct ConstantTex {
    pub color: Vector3<f32>
}

impl Texture for ConstantTex {
    fn value(&self, u:f32, v: f32, p: Vector3<f32>) -> Vector3<f32> {
        self.color
    }
}

pub struct CheckerTex {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>
}

impl Texture for CheckerTex {
    fn value(&self, u:f32, v: f32, p: Vector3<f32>) -> Vector3<f32> {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() *  (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(0.0, 0.0, p)
        } else {
            self.even.value(0.0, 0.0, p)
        }
    }
}