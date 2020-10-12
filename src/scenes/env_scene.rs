use image::{ImageBuffer, Rgb};
use nalgebra::Vector3;
use std::sync::Arc;
use std::{fs, io};

use crate::aarect::{AARect, AARectType::*};
use crate::bvh::BVHNode;
use crate::hittable::{FlipFace, Hittable, Transform};
use crate::material::{Material, Dielectric, DiffuseLight, Lambertian, Metal, Environment, DielectricSurfaceLambert, SimpleEnvironment};
use crate::mesh::Mesh;
// use crate::scenes::prefabs::cornell_box::{
//     cornell_box, cornell_box_camera, cornell_box_environment,
// };
use crate::scenes::Scene;
use crate::sphere::Sphere;
use crate::texture::{ConstantTex, ImageTexture, Sampler::*, WrapMode::*, CheckerTexMap};
use crate::vec::{vec, vec2, vec3, vec_zero};
use crate::camera::{Camera, ApertureShape};


pub fn scene() -> Scene {
    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
    
    let glass = Box::new(Dielectric {
        color: vec(1.0, 1.0, 1.0),
        ref_idx: 1.52,
        ..Dielectric::default()
    });
    let aluminium  = Box::new(Metal { albedo: Box::new(ConstantTex { color: vec3(0.8, 0.85, 0.85) } ), fuzz: 0.0});
    let red = Box::new(Lambertian { albedo: Box::new(ConstantTex { color: vec3(0.65, 0.05, 0.05) })});
    let white = Box::new(Lambertian { albedo: Box::new(ConstantTex { color: vec3(0.73, 0.73, 0.73) })});
    let green = Box::new(Lambertian { albedo: Box::new(ConstantTex { color: vec3(0.12, 0.45, 0.15) })});    

    let checker_tex_map = Box::new(CheckerTexMap {
        odd: ConstantTex::new_arc(vec(0.3, 0.3, 0.3)),
        even: ConstantTex::new_arc(vec(0.9, 0.9, 0.9)),
        scale: 1.0,
    });

    let checker_mat = Box::new(Lambertian { albedo: checker_tex_map });

    let earth_image = image::open("assets/topo.jpg").unwrap().to_rgb();

    let earth_material = Box::new(DielectricSurfaceLambert{
        albedo: Box::new(
            ImageTexture::new(earth_image)
                .sampler(Bilinear)
                .wrap_mode(Clamp),
        ),
        ..DielectricSurfaceLambert::default()
    });

    let env_material = Box::new(Environment::new("assets/wooden_motel_4k.hdr".to_string()));
    let sand = Box::new(Lambertian { albedo: Box::new(ConstantTex { color: vec3(244.0/255.99, 219.0/255.99, 154.0/255.99) })});

    objects.push(Box::new(AARect { 
        xy0: vec2(-1000.0, -1000.0), 
        xy1: vec2(1000.0, 1000.0),
        k: 0.0,
        material: &checker_mat,
        rect_type: XZ
    }));

    // objects.push(Box::new(Transform::new(
    //     Sphere::new(vec_zero(), 200.0, red),
    //     vec3(-450.0, 200.0, -100.0),
    //     vec_zero(),
    // )));

    // objects.push(Box::new(Transform::new(
    //     Sphere::new(vec_zero(), 200.0, white),
    //     vec3(0.0, 200.0, -100.0),
    //     vec_zero(),
    // )));

    // objects.push(Box::new(Transform::new(
    //     Sphere::new(vec_zero(), 200.0, green),
    //     vec3(450.0, 200.0, -100.0),
    //     vec_zero(),
    // )));

    objects.push(Box::new(
        Sphere::new(vec3(-450.0, 200.0, -100.0), 200.0, earth_material),
    ));

    objects.push(Box::new(
        Sphere::new(vec3(0.0, 200.0, -100.0), 200.0, glass),
    ));

    objects.push(Box::new(
        Sphere::new(vec3(450.0, 200.0, -100.0), 200.0, aluminium),
    ));

    // objects.push(Box::new(Transform::new(
    //     Sphere::new(vec_zero(), 100.0, red),
    //     vec3(-450.0, 100.0, -200.0),
    //     vec_zero(),
    // )));


    let lookfrom = vec3(0.0, 500.0, 3500.0);
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
        // environment: env_material,
        environment: Box::new(SimpleEnvironment {}),
        mis_objects: vec![]
    }
}
