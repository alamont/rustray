use nalgebra::Vector3;
use rand::{thread_rng, Rng};
use std::f32;

pub fn random_vec_in_unit_sphere() -> Vector3<f32> {
    let mut rng = thread_rng();
    let mut p;
    loop {
        p = Vector3::new(
            rng.gen_range(-1.0, 1.0),
            rng.gen_range(-1.0, 1.0),
            rng.gen_range(-1.0, 1.0)
        );
        if p.magnitude_squared() < 1.0 {
            break;
        }
    } 
    p
}

pub fn random_unit_vec() -> Vector3<f32>{
    let mut rng = thread_rng();
    let a = rng.gen_range(0.0, 2.0 * f32::consts::PI);
    let z = rng.gen_range(-1.0, 1.0) as f32;
    let r = (1.0 - z*z).sqrt();
    Vector3::new(r*a.cos(), r*a.sin(), z)
}

pub fn deg_to_rad(deg: f32) -> f32{
    deg * f32::consts::PI / 180.0
}