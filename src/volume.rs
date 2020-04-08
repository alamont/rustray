use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::material::{Material, Isotropic};
use crate::texture::{ConstantTex, Texture, CheckerTex};
use crate::ray::Ray;
use crate::vec::{vec2, vec3, vec_one, vec_zero};

use nalgebra::{Vector2, Vector3};
use std::sync::Arc;
use std::f32;
use rand::{thread_rng, Rng};


pub struct ConstantMedium {
    boundary: Box<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: f32
}

impl ConstantMedium {
    pub fn new(boundary: impl Hittable + 'static, density: f32, material: Arc<dyn Material>) -> Self {
        Self {
            boundary: Box::new(boundary),
            phase_function: material,
            neg_inv_density: -1.0 / density
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if ray.albedo_normal_ray {
            return None;
        }
        let mut rng = thread_rng();

        if let Some(mut hit1) = self.boundary.hit(ray, f32::MIN, f32::MAX) {
            if let Some(mut hit2) = self.boundary.hit(ray, hit1.t + 0.0001, f32::MAX) {
                if hit1.t < t_min { hit1.t = t_min; }
                if hit2.t > t_max { hit2.t = t_max; }

                if hit1.t >= hit2.t { return None; }

                if hit1.t < 0.0 { hit1.t = 0.0; }

                let ray_length = ray.direction().magnitude();
                let distance_inside_boundary = (hit2.t - hit1.t) * ray_length;
                let hit_distance = self.neg_inv_density * rng.gen::<f32>().ln();

                if hit_distance > distance_inside_boundary {
                    // Extend ray to check for more hits in concave boundaries
                    return self.hit(ray, hit2.t + 0.0001, f32::MAX);
                }

                let t = hit1.t + hit_distance / ray_length;

                Some(HitRecord::new(
                    t,
                    ray.at(t),
                    vec3(1.0, 0.0, 0.0),
                    ray,
                    Arc::clone(&self.phase_function),
                    vec2(0.0, 0.0)))

            } else { None }
        } else { None }
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.boundary.bounding_box()
    }
}

pub struct NonUniformMedium {
    boundary: Box<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    density: Arc<dyn Texture>,
    max_density: f32,
}

impl NonUniformMedium {
    pub fn new(boundary: impl Hittable + 'static, density: Arc<dyn Texture>, max_density: f32, material: Arc<dyn Material>) -> Self {
        Self {
            boundary: Box::new(boundary),
            phase_function: material,
            density,
            max_density
        }
    }
}

impl Hittable for NonUniformMedium {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if ray.albedo_normal_ray {
            return None;
        }
        let mut rng = thread_rng();

        if let Some(mut hit1) = self.boundary.hit(ray, f32::MIN, f32::MAX) {
            if let Some(mut hit2) = self.boundary.hit(ray, hit1.t + 0.0001, f32::MAX) {
                if hit1.t < t_min { hit1.t = t_min; }
                if hit2.t > t_max { hit2.t = t_max; }

                if hit1.t >= hit2.t { return None; }

                if hit1.t < 0.0 { hit1.t = 0.0; }

                let ray_length = ray.direction().magnitude();
                let distance_inside_boundary = (hit2.t - hit1.t) * ray_length;
                if distance_inside_boundary.is_nan() {
                    return None;
                }

                let s_max = self.max_density;
                let mut d = 0.0;
                let t = loop {
                    let x = rng.gen::<f32>();
                    d += -(1.0 - x).ln() / s_max;
                    let y = rng.gen::<f32>();
                    if d > distance_inside_boundary {
                        break 0.0;
                    }
                    let t = hit1.t + d / ray_length;
                    if self.density.value(hit1.uv, ray.at(t)).x / s_max > y {
                        break t;
                    }
                };

                if d > distance_inside_boundary {
                    // Extend ray to check for more hits in concave boundaries
                    return self.hit(ray, hit2.t + 0.0001, f32::MAX);
                }

                Some(HitRecord::new(
                    t,
                    ray.at(t),
                    vec3(1.0, 0.0, 0.0),
                    ray,
                    Arc::clone(&self.phase_function),
                    vec2(0.0, 0.0)))

            } else { None }
        } else { None }
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.boundary.bounding_box()
    }
}
