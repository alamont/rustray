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


    let earth_image = image::open("assets/topo.jpg").unwrap().to_rgb();
    let decoder = image::hdr::HdrDecoder::new(io::BufReader::new(
        fs::File::open("assets/veranda_8k.hdr").unwrap(),
    ))
    .unwrap();

    let env_image = ImageBuffer::from_raw(
        decoder.metadata().width,
        decoder.metadata().height,
        decoder.read_image_hdr().unwrap().iter().flat_map(|p| vec![p[0], p[1], p[2]]).collect::<Vec<f32>>()
    ).unwrap();

    let earth_material = Arc::new(Lambertian {
        albedo: Arc::new(
            ImageTexture::new(earth_image.clone())
                .sampler(Bilinear)
                .wrap_mode(Clamp),
        ),
    });
    let earth_material_new = Arc::new(DielectricSurfaceLambert{
        albedo: Arc::new(
            ImageTexture::new(earth_image.clone())
                .sampler(Bilinear)
                .wrap_mode(Clamp),
        ),
        ..DielectricSurfaceLambert::default()
    });

    let env_material = Arc::new(Environment {
        emit: Arc::new(
            ImageTexture::new(env_image)
                .sampler(Bilinear)
                .wrap_mode(Clamp),
        ),
    });
    let white = Arc::new(Lambertian { albedo: Arc::new(ConstantTex { color: vec3(0.73, 0.73, 0.73) })});

    objects.push(Arc::new(AARect { 
        xy0: vec2(-10000.0, -10000.0), 
        xy1: vec2(10000.0, 10000.0),
        k: 0.0,
        material: white.clone(),
        rect_type: XZ
    }));

    objects.push(Arc::new(Transform::new(
        Sphere::new(vec_zero(), 200.0, glass.clone()),
        vec3(-450.0, 200.0, -100.0),
        vec_zero(),
    )));

    // This doesn't work properly becuase de dieletric assumes and interface
    // with a medium with ref_idx ~1 (like air)
    // objects.push(Arc::new(Transform::new(
    //     Sphere::new(vec_zero(), 199.999, earth_material),
    //     vec3(0.0, 200.0, -100.0),
    //     vec_zero(),
    // )));
    // objects.push(Arc::new(Transform::new(
    //     Sphere::new(vec_zero(), 200.0, glass.clone()),
    //     vec3(0.0, 200.0, -100.0),
    //     vec_zero(),
    // )));


    objects.push(Arc::new(Transform::new(
        Sphere::new(vec_zero(), 200.0, earth_material_new.clone()),
        vec3(0.0, 200.0, -100.0),
        vec_zero(),
    )));

    objects.push(Arc::new(Transform::new(
        Sphere::new(vec_zero(), 200.0, aluminium.clone()),
        vec3(450.0, 200.0, -100.0),
        vec_zero(),
    )));

    let lookfrom = vec3(0.0, 500.0, 1500.0);
    let lookat = vec3(0.0, 278.0, 0.0);
    let vup = vec3(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom-lookat).magnitude();
    let aperture = 100.0;
    let vfov = 30.0;
    let aspect = 2.0;

    let mut camera = Camera::new(lookfrom, lookat, vup, vfov, aspect, aperture, dist_to_focus);
    camera.aperture_shape = ApertureShape::Hexagon;

    Scene {
        camera,
        objects: BVHNode::build(objects, 0),
        environment: env_material,
        mis_objects: vec![]
    }
}
