use nalgebra::Vector3;
use rand::{thread_rng, Rng};
use hsl::HSL;
use std::sync::Arc;

use crate::hittable::{HittableList, Hittable};
use crate::camera::Camera;
use crate::material::{Dielectric, Lambertian, Metal, SimpleEnvironment};
use crate::sphere::Sphere;
use crate::vec::{vec, random_vec, random_vec_range, vec_zero};
use crate::bvh::BVHNode;
use crate::texture::{ConstantTex, CheckerTex, ImageTexture};
use crate::scenes::Scene;

pub fn random_scene(aspect: f32) -> Scene {
    let mut rng = thread_rng();

    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    let checker_tex = Arc::new(CheckerTex {
        odd: Arc::new(ConstantTex { color: vec(0.2, 0.3, 0.1)}),
        even: Arc::new(ConstantTex { color: vec(0.9, 0.9, 0.9)}),
        scale: 1.0
    });    

    objects.push(Box::new(Sphere{
        center: vec(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(Lambertian {
            albedo: checker_tex,
        }),
    }));


    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f32>();
            let center = Vector3::new(a as f32 + 0.9 * rng.gen::<f32>(), 0.2, b as f32 + 0.9 * rng.gen::<f32>());
            if choose_mat < 0.8 {
                // diffuse
                let albedo = Arc::new(ConstantTex {color: random_vec().component_mul(&random_vec())});
                objects.push(Box::new(Sphere{
                    center, 
                    radius: 0.2, 
                    material: Arc::new(Lambertian{albedo})}));
            } else if choose_mat < 0.95 {
                // metal
                let albedo = random_vec_range(0.5, 1.0);
                let fuzz = rng.gen_range(0.0, 0.5);
                objects.push(Box::new(Sphere{
                    center, 
                    radius: 0.2, 
                    material: Arc::new(Metal{albedo, fuzz})}));
            } else {
                // glass
                let hsl_color = HSL {h: rng.gen_range(0.0, 360.0), s: 1.0, l: 0.95};
                let rgb_color = hsl_color.to_rgb();
                let color = Vector3::new(rgb_color.0 as f32 / 255.0 , rgb_color.1 as f32 / 255.0 , rgb_color.2 as f32 / 255.0 );
                objects.push(Box::new(Sphere{
                    center, 
                    radius: 0.2, 
                    material: Arc::new(Dielectric { 
                        ref_idx: 1.5, 
                        color,
                        density: 0.1,
                        ..Dielectric::default() }),
                    }
                ))
            }
        }
    }

    objects.push(Box::new(Sphere {
        center: vec(-0.4, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Dielectric {
            ref_idx: 1.5, 
            color: vec(1.0, 1.0, 1.0),
            density: 0.1,
            ..Dielectric::default()
        })
    }));
    objects.push(Box::new(Sphere {
        center: vec(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Lambertian {
            albedo: Arc::new(ConstantTex {color: vec(0.4, 0.2, 0.1)})
        })
    }));
    objects.push(Box::new(Sphere {
        center: vec(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Metal {
            albedo: vec(0.7, 0.6, 0.5),
            fuzz: 0.0
        })
    }));
    objects.push(Box::new(Sphere {
        center: Vector3::new(2.5, 0.75, 3.0),
        radius: 0.75,
        material: Arc::new(Lambertian {
            albedo: Arc::new(ImageTexture::new(String::from("assets/earthmap.jpg"))),
        }),
    }));


    let lookfrom = vec(12.0, 2.0, 3.0);
    let lookat = vec_zero();
    let vup = vec(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;


    Scene {
        camera: Camera::new(lookfrom, lookat, vup, 20.0, aspect, aperture, dist_to_focus),
        objects: BVHNode::build(objects, 0),
        environment: Arc::new(SimpleEnvironment {})
    }
}