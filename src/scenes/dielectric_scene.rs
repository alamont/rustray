use hsl::HSL;
use nalgebra::{Vector2, Vector3};
use rand::{thread_rng, Rng};
use std::sync::Arc;

use crate::bvh::BVHNode;
use crate::camera::Camera;
use crate::hittable::{Hittable, HittableList};
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::Sphere;
use crate::texture::{CheckerTex, CheckerTexMap, ConstantTex};
use crate::vec::{vec, vec_zero};

pub fn dielectric_scene(aspect: f32) -> (Camera,Box<dyn Hittable>) {
    let mut world = HittableList::default();

    let checker_floor = Box::new(CheckerTex {
        odd: ConstantTex::new_arc(vec(0.2, 0.3, 0.1)),
        even: ConstantTex::new_arc(vec(0.6, 0.6, 0.6)),
        scale: 1.0,
    });

    let checker_tex = Box::new(CheckerTex {
        odd: ConstantTex::new_arc(vec(0.3, 0.3, 0.3)),
        even: ConstantTex::new_arc(vec(0.9, 0.9, 0.9)),
        scale: 1.0,
    });

    let checker_tex_map = Box::new(CheckerTexMap {
        odd: ConstantTex::new_arc(vec(0.3, 0.3, 0.3)),
        even: ConstantTex::new_arc(vec(0.9, 0.9, 0.9)),
        scale: 0.25,
    });


    
    world.push(Sphere {
        center: Vector3::new(0.0, -1000.5, -1.0),
        radius: 1000.0,
        material: Box::new(Lambertian {
            albedo: checker_floor.clone(),
        }),
    });


    // let checker_roughness = Box::new(CheckerTex {
    //     odd: ConstantTex::new_arc(vec(1.0, 0.1, 0.1)),
    //     even: ConstantTex::new_arc(vec_zero())),
    //     scale: 0.5,
    // }});

    // Sphere 1
    world.push(Sphere {
        center: Vector3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Box::new(Dielectric {
            color: vec(0.1, 0.1, 1.0),
            density: 0.1,
            ..Dielectric::default()
        }),
    });

    world.push(Sphere {
        center: Vector3::new(-1.0, 0.0, -1.0),
        radius: -0.45,
        material: Box::new(Dielectric {
            color: vec(0.1, 0.1, 1.0),
            density: 0.1,
            ..Dielectric::default()
        }),
    });

    world.push(Sphere {
        center: Vector3::new(-1.0, 0.0, -1.0),
        radius: 0.2,
        material: Box::new(Dielectric {
            color: vec(0.1, 0.1, 1.0),
            density: 0.1,
            ..Dielectric::default()
        }),
    });

    //Sphere 2
    world.push(Sphere {
        center: Vector3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Box::new(Lambertian {
            albedo: checker_tex_map,
        }),
    });
    
    // Sphere 3
    world.push(Sphere {
        center: Vector3::new(0.0, 0.0, -2.0),
        radius: 0.5,
        material: Box::new(Lambertian {
            albedo: checker_tex.clone(),
        }),
    });

    // Sphere 4
    world.push(Sphere {
        center: Vector3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Box::new(Dielectric {
            color: vec(1.0, 1.0, 1.0),
            ..Dielectric::default()
        }),
    });


    let lookfrom = vec(1.0, 4.0, 4.0);
    let lookat = vec(0.0, 0.0, -1.0);
    let vup = vec(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).magnitude();
    let aperture = 0.01;

    (
        Camera::new(lookfrom, lookat, vup, 20.0, aspect, aperture, dist_to_focus),
        Box::new(world),
    )
}
