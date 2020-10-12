use image::{ImageBuffer, Rgb};
use nalgebra::Vector3;
use std::sync::Arc;
use std::{fs, io};

use crate::aarect::{AARect, AARectType::*};
use crate::bvh::BVHNode;
use crate::hittable::{FlipFace, Hittable, Transform};
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::mesh::Mesh;
use crate::scenes::prefabs::cornell_box::{
    cornell_box, cornell_box_camera, cornell_box_environment,
};
use crate::scenes::Scene;
use crate::sphere::Sphere;
use crate::texture::{ConstantTex, ImageTexture, Sampler::*, WrapMode::*};
use crate::vec::{vec, vec2, vec3, vec_zero};

pub fn scene() -> Scene {
    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    objects.push(cornell_box());
    let glass = Box::new(Dielectric {
        color: vec(1.0, 1.0, 1.0),
        ..Dielectric::default()
    });

    let earth_image = image::open("assets/topo.jpg").unwrap().to_rgb();
    // let decoder = image::hdr::HdrDecoder::new(io::BufReader::new(
    //     fs::File::open("assets/umhlanga_sunrise_4k.hdr").unwrap(),
    // ))
    // .unwrap();

    // let earth_image = ImageBuffer::from_raw(
    //     decoder.metadata().width,
    //     decoder.metadata().height,
    //     decoder.read_image_hdr().unwrap().iter().flat_map(|p| vec![p[0], p[1], p[2]]).collect::<Vec<f32>>()
    // ).unwrap();

    let earth_material = Box::new(Lambertian {
        albedo: Box::new(
            ImageTexture::new(earth_image)
                .sampler(Bilinear)
                .wrap_mode(Clamp),
        ),
    });
    let small_light = Box::new(DiffuseLight {
        emit: Box::new(ConstantTex {
            color: vec3(6.0, 6.0, 6.0),
        }),
    });

    objects.push(Box::new(Transform::new_b(
        Box::new(FlipFace::new(AARect {
            xy0: vec2(-50.0, -50.0),
            xy1: vec2(50.0, 50.0),
            k: 0.0,
            material: small_light.clone(),
            rect_type: XY,
        })),
        vec3(-200.0, 50.0, 500.0),
        vec3(20.0, -20.0, 0.0),
    )));

    objects.push(Box::new(Transform::new_b(
        Box::new(FlipFace::new(AARect {
            xy0: vec2(-50.0, -50.0),
            xy1: vec2(50.0, 50.0),
            k: 0.0,
            material: small_light.clone(),
            rect_type: XY,
        })),
        vec3(200.0, 500.0, 500.0),
        vec3(-20.0, 20.0, 0.0),
    )));

    objects.push(Box::new(Transform::new(
        Sphere::new(vec_zero(), 199.999, earth_material),
        vec3(0.0, 200.0, -100.0),
        vec_zero(),
    )));

    objects.push(Box::new(Transform::new(
        Sphere::new(vec_zero(), 200.0, glass),
        vec3(0.0, 200.0, -100.0),
        vec_zero(),
    )));

    Scene {
        camera: cornell_box_camera(),
        objects: BVHNode::build(objects, 0),
        environment: cornell_box_environment(),
    }
}
