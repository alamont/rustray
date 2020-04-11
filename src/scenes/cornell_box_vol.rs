use nalgebra::{Vector2, Vector3};
use std::sync::Arc;

use crate::hittable::{HittableList, Hittable, FlipFace, Transform};
use crate::camera::Camera;
use crate::material::{
    Dielectric, 
    Lambertian, 
    Metal, 
    Environment, 
    DiffuseLight,
    Isotropic
};
use crate::volume::{ConstantMedium, NonUniformMedium};
use crate::aarect::{AARect, AARectType::*};
use crate::aabox::AABox;
use crate::vec::{vec, vec2, vec3, vec_one, random_vec, random_vec_range, vec_zero};
use crate::bvh::BVHNode;
use crate::texture::{ConstantTex, CheckerTex, ImageTexture};
use crate::scenes::Scene;

pub fn cornell_box_vol(aspect: f32) -> Scene {

    let red = Arc::new(Lambertian { albedo: Arc::new(ConstantTex { color: vec3(0.65, 0.05, 0.05) })});
    let white = Arc::new(Lambertian { albedo: Arc::new(ConstantTex { color: vec3(0.73, 0.73, 0.73) })});
    let green = Arc::new(Lambertian { albedo: Arc::new(ConstantTex { color: vec3(0.12, 0.45, 0.15) })});
    // let light = Arc::new(DiffuseLight { emit: Arc::new(ConstantTex { color: vec3(14.0, 14.0, 14.0) })});
    let light = Arc::new(DiffuseLight { emit: Arc::new(ConstantTex { color: vec3(7.0, 7.0, 7.0) })});
    let aluminium  = Arc::new(Metal { albedo: Arc::new(ConstantTex { color: vec3(0.8, 0.85, 0.85) } ), fuzz: 0.0});

    let dark_medium = Arc::new(Isotropic { albedo: Arc::new(ConstantTex { color: vec_zero() })});
    let light_medium = Arc::new(Isotropic { albedo: Arc::new(ConstantTex { color: vec_one() })});

    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    objects.push(Box::new(FlipFace::new(AARect { 
        xy0: vec2(0.0, 0.0), 
        xy1: vec2(555.0, 555.0),
        k: 555.0,
        material: green.clone(),
        rect_type: YZ
    })));
    objects.push(Box::new(AARect { 
        xy0: vec2(0.0, 0.0), 
        xy1: vec2(555.0, 555.0),
        k: 0.0,
        material: red.clone(),
        rect_type: YZ
    }));
    objects.push(Box::new(AARect { 
        xy0: vec2(0.0, 0.0), 
        xy1: vec2(555.0, 555.0),
        k: 555.0,
        material: white.clone(),
        rect_type: XZ
    }));
    // // Original light
    // objects.push(Box::new(FlipFace::new(AARect { 
    //     xy0: vec2(213.0, 227.0), 
    //     xy1: vec2(343.0, 332.0),
    //     k: 554.0,
    //     material: light.clone(),
    //     rect_type: XZ
    // })));
    // Bigger light
    objects.push(Box::new(FlipFace::new(AARect { 
        xy0: vec2(113.0, 127.0), 
        xy1: vec2(443.0, 442.0),
        k: 554.0,
        material: light.clone(),
        rect_type: XZ
    })));
    objects.push(Box::new(AARect { 
        xy0: vec2(0.0, 0.0), 
        xy1: vec2(555.0, 555.0),
        k: 0.0,
        material: white.clone(),
        rect_type: XZ
    }));
    objects.push(Box::new(FlipFace::new(AARect { 
        xy0: vec2(0.0, 0.0), 
        xy1: vec2(555.0, 555.0),
        k: 555.0,
        material: white.clone(),
        rect_type: XY
    })));


    let box1 = Transform::new(
        AABox::new(
            vec3(165.0, 165.0, 165.0), 
            white.clone()
        ),
        vec3(115.0 + 165.0/2.0, 165.0/2.0, 65.0 + 165.0/2.0),
        vec3(0.0, -18.0, 0.0)
    );
    // let box2 = Transform::new(
    //     AABox::new(
    //         vec(165.0, 330.0, 165.0),
    //         white.clone()
    //     ),
    //     vec3(280.0 + 165.0/2.0, 330.0/2.0, 295.0 + 165.0/2.0),
    //     vec3(0.0, 15.0, 0.0)
    // );
    let box2 = Transform::new(
        AABox::new(
            vec(165.0, 330.0, 165.0),
            aluminium.clone()
        ),
        vec3(280.0 + 165.0/2.0, 330.0/2.0, 295.0 + 165.0/2.0),
        vec3(0.0, 15.0, 0.0),
    );

    let box1_no_transform = AABox::new(
        vec(165.0, 165.0, 165.0),
        white.clone()
    );
    let box1_density_texture = Arc::new(CheckerTex {
        odd: Arc::new(ConstantTex {color: vec_one() * 0.02}),
        even: Arc::new(ConstantTex {color: vec_zero()}),
        scale: 300.0,
    });


    // objects.push(Box::new(NonUniformMedium::new(box1, 0.05, light_medium)));
    objects.push(Box::new(ConstantMedium::new(box2, 0.02, dark_medium)));

    // To get local coordinates (before transform) for mediums, 
    // we need to apply the transform om the medium instead of the boundary
    objects.push(Box::new(
        Transform::new(
            NonUniformMedium::new(box1_no_transform, box1_density_texture, 0.02, light_medium),
            vec3(115.0 + 165.0/2.0, 165.0/2.0, 65.0 + 165.0/2.0),
            vec3(0.0, -18.0, 0.0)
        )
    ));



    let lookfrom = vec3(278.0, 278.0, -800.0);
    let lookat = vec3(278.0, 278.0, 0.0);
    let vup = vec3(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 40.0;

    Scene {
        camera: Camera::new(lookfrom, lookat, vup, vfov, aspect, aperture, dist_to_focus),
        objects: BVHNode::build(objects, 0),
        environment: Arc::new(Environment { emit: Arc::new(ConstantTex { color: vec_zero() })})
    }
}