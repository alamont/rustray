use image::{ImageBuffer, Rgb};
use nalgebra::Vector3;
use std::sync::Arc;
use std::{fs, io};

use crate::aarect::{AARect, AARectType::*};
use crate::bvh::BVHNode;
use crate::hittable::{FlipFace, Hittable, Transform};
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal, Environment, DielectricSurfaceLambert};
use crate::mesh::Mesh;
// use crate::scenes::prefabs::cornell_box::{
//     cornell_box, cornell_box_camera, cornell_box_environment,
// };
use crate::scenes::Scene;
use crate::sphere::Sphere;
use crate::texture::{ConstantTex, ImageTexture, Sampler::*, WrapMode::*};
use crate::vec::{vec, vec2, vec3, vec_zero};
use crate::camera::{Camera, ApertureShape};


pub fn scene() -> Scene {
    let mut objects: Vec<Arc<dyn Hittable>> = Vec::new();

    let glass = Arc::new(Dielectric {
        color: vec(1.0, 1.0, 1.0),
        ..Dielectric::default()
    });
    let aluminium  = Arc::new(Metal { albedo: Arc::new(ConstantTex { color: vec3(0.8, 0.85, 0.85) } ), fuzz: 0.0});
    let red = Arc::new(Lambertian { albedo: Arc::new(ConstantTex { color: vec3(0.65, 0.05, 0.05) })});
    let white = Arc::new(Lambertian { albedo: Arc::new(ConstantTex { color: vec3(0.73, 0.73, 0.73) })});
    let green = Arc::new(Lambertian { albedo: Arc::new(ConstantTex { color: vec3(0.12, 0.45, 0.15) })});    

    let earth_image = image::open("assets/topo.jpg").unwrap().to_rgb();

    let earth_material = Arc::new(DielectricSurfaceLambert{
        albedo: Arc::new(
            ImageTexture::new(earth_image.clone())
                .sampler(Bilinear)
                .wrap_mode(Clamp),
        ),
        ..DielectricSurfaceLambert::default()
    });

    let env_material = Arc::new(Environment::new("assets/spiaggia_di_mondello_4k.hdr".to_string()));
    let sand = Arc::new(Lambertian { albedo: Arc::new(ConstantTex { color: vec3(244.0/255.99, 219.0/255.99, 154.0/255.99) })});

    objects.push(Arc::new(AARect { 
        xy0: vec2(-10000.0, -10000.0), 
        xy1: vec2(10000.0, 10000.0),
        k: 0.0,
        material: sand.clone(),
        rect_type: XZ
    }));

    // objects.push(Arc::new(Transform::new(
    //     Sphere::new(vec_zero(), 200.0, red.clone()),
    //     vec3(-450.0, 200.0, -100.0),
    //     vec_zero(),
    // )));

    // objects.push(Arc::new(Transform::new(
    //     Sphere::new(vec_zero(), 200.0, white.clone()),
    //     vec3(0.0, 200.0, -100.0),
    //     vec_zero(),
    // )));

    // objects.push(Arc::new(Transform::new(
    //     Sphere::new(vec_zero(), 200.0, green.clone()),
    //     vec3(450.0, 200.0, -100.0),
    //     vec_zero(),
    // )));

    objects.push(Arc::new(
        Sphere::new(vec3(-450.0, 200.0, -100.0), 200.0, glass.clone()),
    ));

    objects.push(Arc::new(
        Sphere::new(vec3(0.0, 200.0, -100.0), 200.0, earth_material.clone()),
    ));

    objects.push(Arc::new(
        Sphere::new(vec3(450.0, 200.0, -100.0), 200.0, aluminium.clone()),
    ));

    let lookfrom = vec3(0.0, 1200.0, 4500.0);
    let lookat = vec3(0.0, 278.0, 0.0);
    let vup = vec3(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom-lookat).magnitude();
    let aperture = 50.0;
    let vfov = 10.0;
    let aspect = 2.0;

    let mut camera = Camera::new(lookfrom, lookat, vup, vfov, aspect, aperture, dist_to_focus);
    camera.aperture_shape = ApertureShape::Circle;

    Scene {
        camera,
        objects: BVHNode::build(objects, 0),
        environment: env_material,
        mis_objects: vec![]
    }
}
