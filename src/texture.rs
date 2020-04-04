use nalgebra::{Vector2, Vector3};
use std::sync::Arc;
use crate::hittable::HitRecord;



pub trait Texture: Sync + Send {
    fn value(&self, uv: Vector2<f32>, p: Vector3<f32>) -> Vector3<f32>;
}

pub struct ConstantTex {
    pub color: Vector3<f32>
}

impl Texture for ConstantTex {
    fn value(&self, _uv: Vector2<f32>, p: Vector3<f32>) -> Vector3<f32> {
        self.color
    }
}

impl ConstantTex {
    pub fn new_arc(color: Vector3<f32>) -> Arc<dyn Texture>{
        Arc::new(Self { color })
    }
}


pub struct CheckerTex {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>, 
    pub scale: f32,
}

impl Texture for CheckerTex {
    fn value(&self, uv: Vector2<f32>, p: Vector3<f32>) -> Vector3<f32> {
        let sines = (10.0 * p.x / self.scale).sin() * (10.0 * p.y / self.scale).sin() *  (10.0 * p.z / self.scale).sin();
        if sines < 0.0 {
            self.odd.value(uv, p)
        } else {
            self.even.value(uv, p)
        }
    }
}

pub struct CheckerTexMap {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
    pub scale: f32,
}

impl Texture for CheckerTexMap {
    fn value(&self, uv: Vector2<f32>, p: Vector3<f32>) -> Vector3<f32> {
        let sines = (10.0 * uv.x / self.scale).sin() * (10.0 * uv.y / self.scale).sin();
        if sines < 0.0 {
            self.odd.value(uv, p)
        } else {
            self.even.value(uv, p)
        }
    }
}

pub struct ImageTexture {
    data: Vec<u8>,
    nx: u32,
    ny: u32
}

impl ImageTexture {
    pub fn new(image_path: String) -> Self {
        let image = image::open(image_path.as_str()).expect("Can't find image").to_rgb();
        let (nx, ny) = image.dimensions();
        let pixels = image.into_raw();
        ImageTexture {
            data: pixels,
            nx,
            ny
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, uv: Vector2<f32>, p: Vector3<f32>) -> Vector3<f32> {        
        let mut i = (uv.x * self.nx as f32) as u32;
        let mut j = ((1.0 - uv.y) * self.ny as f32 - 0.001) as u32;

        if i < 0 {i = 0}
        if j < 0 {j = 0}
        if i > self.nx-1 {i = self.nx-1}
        if j > self.ny-1 {j = self.ny-1}

        let r = self.data[(3 * i + 3 * self.nx * j + 0) as usize] as f32 / 255.0;
        let g = self.data[(3 * i + 3 * self.nx * j + 1) as usize] as f32 / 255.0;
        let b = self.data[(3 * i + 3 * self.nx * j + 2) as usize] as f32 / 255.0;

        Vector3::new(r, g, b)
    }
}