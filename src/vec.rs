use nalgebra::{Vector2, Vector3};
use rand::{thread_rng, Rng};
use std::f32;

pub fn random_vec_in_unit_sphere() -> Vector3<f32> {
    let mut rng = thread_rng();
    let mut p;
    loop {
        p = Vector3::new(
            rng.gen_range(-1.0, 1.0),
            rng.gen_range(-1.0, 1.0),
            rng.gen_range(-1.0, 1.0),
        );
        if p.magnitude_squared() < 1.0 {
            break;
        }
    }
    p
}

pub fn random_unit_vec() -> Vector3<f32> {
    let mut rng = thread_rng();
    let a = rng.gen_range(0.0, 2.0 * f32::consts::PI);
    let z = rng.gen_range(-1.0, 1.0) as f32;
    let r = (1.0 - z * z).sqrt();
    Vector3::new(r * a.cos(), r * a.sin(), z)
}

pub fn random_unit_in_disk() -> Vector3<f32> {
    let mut rng = thread_rng();
    let mut p;
    loop {
        p = Vector3::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0);
        if p.magnitude_squared() < 1.0 {
            break;
        }
    }
    p
}

pub fn deg_to_rad(deg: f32) -> f32 {
    deg * f32::consts::PI / 180.0
}

pub fn vec3(x: f32, y: f32, z: f32) -> Vector3<f32> {
    Vector3::new(x, y, z)
}

//TODO: Refactor vec to vec3
pub fn vec(x: f32, y: f32, z: f32) -> Vector3<f32> {
    Vector3::new(x, y, z)
}

pub fn vec2(x: f32, y: f32) -> Vector2<f32> {
    Vector2::new(x, y)
}

pub fn vec_zero() -> Vector3<f32> {
    Vector3::new(0.0, 0.0, 0.0)
}

pub fn vec_one() -> Vector3<f32> {
    Vector3::new(1.0, 1.0, 1.0)
}

pub fn random_vec() -> Vector3<f32> {
    let mut rng = thread_rng();
    Vector3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
}

pub fn random_vec_range(a: f32, b: f32) -> Vector3<f32> {
    let mut rng = thread_rng();
    Vector3::new(
        rng.gen_range(a, b),
        rng.gen_range(a, b),
        rng.gen_range(a, b),
    )
}

pub fn has_nan(v: &Vector3<f32>) -> bool {
    v.x.is_nan() || v.y.is_nan() || v.z.is_nan()
}

pub fn onb_local(w: &Vector3<f32>, direction: &Vector3<f32>) -> Vector3<f32> {
    let a = if w.x.abs() > 0.9 {
        Vector3::new(0.0, 1.0, 0.0)
    } else {
        Vector3::new(1.0, 0.0, 0.0)
    };
    let v = w.cross(&a).normalize();
    let u = w.cross(&v);
    direction.x * u + direction.y * v + direction.z * w
}

pub fn random_cosine_direction() -> Vector3<f32> {
    let mut rng = thread_rng();

    let r1 = rng.gen::<f32>();
    let r2 = rng.gen::<f32>();
    let z = (1.0 - r2).sqrt();

    let phi = 2.0 * f32::consts::PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    vec3(x, y, z)
}


pub fn get_sphere_uv(p: &Vector3<f32>) -> Vector2<f32> {
    let phi = p.z.atan2(p.x);
    let theta = p.y.asin();
    let u = 1.0 - (phi + f32::consts::PI) / (2.0 * f32::consts::PI);
    let v = (theta + f32::consts::PI / 2.0) / f32::consts::PI;
    Vector2::new(u, v)
}

