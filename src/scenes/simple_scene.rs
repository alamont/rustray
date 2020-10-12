use nalgebra::Vector3;
use rand::{thread_rng, Rng};
use hsl::HSL;
use std::sync::Arc;

use crate::hittable::{HittableList, Hittable};
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::Sphere;
use crate::vec::{vec, random_vec, random_vec_range};
use crate::bvh::BVHNode;

pub fn simple_scene() -> HittableList {
    let mut world = HittableList::default();
    world.push(Sphere {
        center: Vector3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Box::new(Lambertian {
            albedo: Vector3::new(0.1, 0.2, 0.5),
        }),
    });
    world.push(Sphere {
        center: Vector3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Box::new(Lambertian {
            albedo: Vector3::new(0.8, 0.8, 0.0),
        }),
    });

    world.push(Sphere {
        center: Vector3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Box::new(Metal {
            albedo: Vector3::new(0.8, 0.6, 0.2),
            fuzz: 0.0,
        }),
    });
    world.push(Sphere {
        center: Vector3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Box::new(Dielectric { ref_idx: 1.5, reflection_color: vec(1.0, 0.9, 0.9), refraction_color: vec(1.0, 0.9, 0.9) }),
    });
    // world.push(Sphere {
    //     center: Vector3::new(-1.0, 0.0, -1.0),
    //     radius: -0.45,
    //     material: Box::new(Dielectric { ref_idx: 1.5 }),
    // });
    world
}