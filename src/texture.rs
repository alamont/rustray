use nalgebra::{Vector2, Vector3, Vector4};
use std::sync::Arc;

use crate::hittable::HitRecord;
use crate::vec::vec_zero;
use crate::utils::clamp;

pub trait Texture: Sync + Send {
    fn value(&self, uv: Vector2<f32>, p: Vector3<f32>) -> Vector3<f32>;
}

pub struct ConstantTex {
    pub color: Vector3<f32>,
}

impl Texture for ConstantTex {
    fn value(&self, _uv: Vector2<f32>, _p: Vector3<f32>) -> Vector3<f32> {
        self.color
    }
}

impl ConstantTex {
    pub fn new_arc(color: Vector3<f32>) -> Arc<dyn Texture> {
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
        let sines = (10.0 * p.x / self.scale).sin()
            * (10.0 * p.y / self.scale).sin()
            * (10.0 * p.z / self.scale).sin();
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

pub enum Sampler {
    Nearest,
    Bilinear,
    Bicubic,
}

pub enum WrapMode {
    Clamp,
    Repeat,
    Mirror,
}

pub struct ImageTexture {
    data: Vec<u8>,
    width: u32,
    height: u32,
    _sampler: Sampler, 
    // TODO: Add Texture Format. e.g. RGB, RGBA
    _wrap_mode: WrapMode,
}

impl ImageTexture {
    pub fn new(image_path: String) -> Self {
        use Sampler::*;
        use WrapMode::*;
        let image = image::open(image_path.as_str())
            .expect("Can't find image")
            .to_rgb();
        let (width, height) = image.dimensions();
        let pixels = image.into_raw();
        ImageTexture {
            data: pixels,
            width,
            height,
            _sampler: Nearest,
            _wrap_mode: Clamp,
        }
    }
    pub fn sampler(mut self, sampler: Sampler) -> Self {
        self._sampler = sampler;
        self
    }
    pub fn wrap_mode(mut self, wrap_mode: WrapMode) -> Self {
        self._wrap_mode = wrap_mode;
        self
    }
}

impl Texture for ImageTexture {
    fn value(&self, uv: Vector2<f32>, _p: Vector3<f32>) -> Vector3<f32> {
        use Sampler::*;
        use WrapMode::*;

        let mut u = uv.x;
        let mut v = uv.y;

        match self._wrap_mode {
            Clamp => {
                u = clamp(u, 0.0, 1.0);
                v = clamp(v, 0.0, 1.0);
            },
            Repeat => {
                u = u % 1.0;
                v = v % 1.0;
            },
            Mirror => {
                u = ((u + 1.0) % 2.0 - 1.0).abs();
                v = ((v + 1.0) % 2.0 - 1.0).abs();
            }
        }


        let x = u * (self.width - 1) as f32;
        let y = (1.0 - v) * (self.height - 1) as f32;


        let i = x as u32;
        let j = y as u32;
        // if i > self.width - 1 {
        //     i = self.width - 1
        // }
        // if j > self.height - 1 {
        //     j = self.height - 1
        // }

        match &self._sampler {
            Nearest => self.get_pixel(i, j),
            Bilinear => {
                let px = x.fract();
                let py = y.fract();

                let p1 = self.get_pixel(x.floor() as u32, y.floor() as u32); //p0[0 + 0 * stride];
                let p2 = self.get_pixel(x.ceil() as u32, y.floor() as u32); //p0[1 + 0 * stride];
                let p3 = self.get_pixel(x.floor() as u32, y.ceil() as u32); //p0[0 + 1 * stride];
                let p4 = self.get_pixel(x.ceil() as u32, y.ceil() as u32); //p0[1 + 1 * stride];

                let w1 = (1.0 - px) * (1.0 - py);
                let w2 = px * (1.0 - py);
                let w3 = (1.0 - px) * py;
                let w4 = px * py;
                let w = Vector4::new(w1, w2, w3, w4);

                let r = Vector4::new(p1.x, p2.x, p3.x, p4.x).dot(&w);
                let g = Vector4::new(p1.y, p2.y, p3.y, p4.y).dot(&w);
                let b = Vector4::new(p1.z, p2.z, p3.z, p4.z).dot(&w);

                Vector3::new(r, g, b)
            }
            _ => vec_zero(),
        }
    }
}

impl ImageTexture {
    fn get_pixel(&self, i: u32, j: u32) -> Vector3<f32> {
        let r = self.data[(3 * i + 3 * self.width * j + 0) as usize] as f32 / 255.0;
        let g = self.data[(3 * i + 3 * self.width * j + 1) as usize] as f32 / 255.0;
        let b = self.data[(3 * i + 3 * self.width * j + 2) as usize] as f32 / 255.0;
        Vector3::new(r, g, b)
    }
}
