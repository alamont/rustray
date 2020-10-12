use crate::hittable::Hittable;
use crate::vec::{onb_local, random_cosine_direction, vec3, vec_zero, random_unit_vec};
use crate::material::EnvironmentMaterial;
use nalgebra::{Vector2, Vector3};
use rand::{thread_rng, Rng};
use std::{f32, sync::Arc};

pub trait Pdf: Sync + Send {
    fn value(&self, _direction: Vector3<f32>) -> f32 {
        0.0
    }
    fn generate(&self) -> Vector3<f32> {
        vec_zero()
    }
}

pub struct CosinePdf {
    pub w: Vector3<f32>,
}

impl CosinePdf {
    pub fn new(w: Vector3<f32>) -> Self {
        Self { w: w.normalize() }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: Vector3<f32>) -> f32 {
        let cosine = direction.normalize().dot(&self.w);
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / f32::consts::PI
        }
    }
    fn generate(&self) -> Vector3<f32> {
        onb_local(&self.w, &random_cosine_direction())
    }
}

pub struct UniformPdf { }
impl Pdf for UniformPdf { 
    fn value(&self, _direction: Vector3<f32>) -> f32 {
        1.0 / (4.0 * f32::consts::PI)
    }
    fn generate(&self) -> Vector3<f32> {
        random_unit_vec()
    }
}

pub struct HittablePdf<'a> {
    pub origin: Vector3<f32>,
    pub hittable: &'a Box<dyn Hittable>,
}

impl<'a> Pdf for HittablePdf<'a> {
    fn value(&self, direction: Vector3<f32>) -> f32 {
        self.hittable.pdf_value(&self.origin, &direction)
    }
    fn generate(&self) -> Vector3<f32> {
        self.hittable.random(&self.origin)
    }
}

pub struct EnvPdf { 
    pub environment: Box<dyn EnvironmentMaterial>
}

impl Pdf for EnvPdf {
    fn value(&self, direction: Vector3<f32>) -> f32 {
        self.environment.pdf_value(&direction)
    }
    fn generate(&self) -> Vector3<f32> {
        self.environment.random()
    }
}

pub struct MixturePdf {
    pub pdfs: Vec<Box<dyn Pdf>>,
    // pub weights: Vec<f32>,
    // pub cum_weights: Vec<f32>
}

impl MixturePdf {
    pub fn new_uniform(pdfs: Vec<Box<dyn Pdf>>) -> Self {
        // let weights = vec![1.0 / pdfs.len() as f32; pdfs.len()];
        // let acc_init: Vec<f32> = Vec::new();
        // let cum_weights = weights.iter().fold(acc_init, |mut acc, w| {
        //     if acc.len() > 0 {
        //         acc.push(acc.last().unwrap() + w)
        //     } else {
        //         acc.push(*w)
        //     }
        //     acc
        // });
        Self {
            pdfs,
            // weights,
            // cum_weights
        }
    }
}

impl Pdf for MixturePdf {
    fn value(&self, direction: Vector3<f32>) -> f32 {
        let weight = 1.0 / self.pdfs.len() as f32;
        self.pdfs.iter().fold(0.0, |acc, p| {
            acc + p.value(direction) * weight
        })
    }
    fn generate(&self) -> Vector3<f32> {
        let mut rng = thread_rng();
        let r = rng.gen_range(0, self.pdfs.len());
        let pdf_idx = r as usize;
        self.pdfs[pdf_idx].generate()
    }
}
