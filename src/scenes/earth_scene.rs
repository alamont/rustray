use hsl::HSL;
use nalgebra::{Vector2, Vector3};
use rand::{thread_rng, Rng};
use std::sync::Arc;

use crate::bvh::BVHNode;
use crate::camera::Camera;
use crate::hittable::{Hittable, HittableList};
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::Sphere;
use crate::texture::{CheckerTex, CheckerTexMap, ConstantTex, ImageTexture};
use crate::vec::{vec, vec_zero};

pub fn earth_scene(aspect: f32) -> (Camera, Box<dyn Hittable>) {
    let mut world = HittableList::default();

    let checker_tex = Arc::new(CheckerTex {
        odd: ConstantTex::new_arc(vec(0.3, 0.3, 0.3)),
        even: ConstantTex::new_arc(vec(0.9, 0.9, 0.9)),
        scale: 1.0,
    });

    let checker_tex_map = Arc::new(CheckerTexMap {
        odd: ConstantTex::new_arc(vec(0.3, 0.3, 0.3)),
        even: ConstantTex::new_arc(vec(0.9, 0.9, 0.9)),
        scale: 0.25,
    });


    
    world.push(Sphere {
        center: Vector3::new(0.0, -1000.5, -1.0),
        radius: 1000.0,
        material: Arc::new(Lambertian {
            albedo: ConstantTex::new_arc(vec(0.5, 0.5, 0.5)),
        }),
    });

    world.push(Sphere {
        center: Vector3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Arc::new(Lambertian {
            albedo: checker_tex_map,
        }),
    });

    world.push(Sphere {
        center: Vector3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Arc::new(Lambertian {
            albedo: checker_tex,
        }),
    });


    world.push(Sphere {
        center: Vector3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Arc::new(Lambertian {
            albedo: Arc::new(ImageTexture::new(String::from("assets/earthmap.jpg"))),
        }),
    });



    let lookfrom = vec(1.0, 4.0, 3.0);
    let lookat = vec(0.0, 0.0, -1.0);
    let vup = vec(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).magnitude();
    let aperture = 0.01;

    (
        Camera::new(lookfrom, lookat, vup, 20.0, aspect, aperture, dist_to_focus),
        Box::new(world),
    )
}
