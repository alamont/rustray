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
        material: Arc::new(Lambertian {
            albedo: Vector3::new(0.1, 0.2, 0.5),
        }),
    });
    world.push(Sphere {
        center: Vector3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Arc::new(Lambertian {
            albedo: Vector3::new(0.8, 0.8, 0.0),
        }),
    });

    world.push(Sphere {
        center: Vector3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Arc::new(Metal {
            albedo: Vector3::new(0.8, 0.6, 0.2),
            fuzz: 0.0,
        }),
    });
    world.push(Sphere {
        center: Vector3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Arc::new(Dielectric { ref_idx: 1.5, reflection_color: vec(1.0, 0.9, 0.9), refraction_color: vec(1.0, 0.9, 0.9) }),
    });
    // world.push(Sphere {
    //     center: Vector3::new(-1.0, 0.0, -1.0),
    //     radius: -0.45,
    //     material: Arc::new(Dielectric { ref_idx: 1.5 }),
    // });
    world
}

pub fn random_scene() -> HittableList {
    let mut world = HittableList::default();
    let mut rng = thread_rng();

    world.push(Sphere{
        center: vec(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(Lambertian {
            albedo: vec(0.5, 0.5, 0.5),
        }),
    });

    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    for a in -41..41 {
        for b in -41..41 {
            let choose_mat = rng.gen::<f32>();
            let center = Vector3::new(a as f32 + 0.5 * rng.gen::<f32>(), 0.2, b as f32 + 0.5 * rng.gen::<f32>());
            if choose_mat < 0.8 {
                // diffuse
                let albedo = random_vec().component_mul(&random_vec());
                world.push(Sphere{
                    center, 
                    radius: 0.2, 
                    material: Arc::new(Lambertian{albedo})});
            } else if choose_mat < 0.95 {
                // metal
                let albedo = random_vec_range(0.5, 1.0);
                let fuzz = rng.gen_range(0.0, 0.5);
                world.push(Sphere{
                    center, 
                    radius: 0.2, 
                    material: Arc::new(Metal{albedo, fuzz})})
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
                        reflection_color: color, 
                        refraction_color: color }),
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
            reflection_color: vec(1.0, 1.0, 1.0),
            refraction_color: vec(1.0, 1.0, 1.0),
        })
    }));
    objects.push(Box::new(Sphere {
        center: vec(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Lambertian {
            albedo: vec(0.4, 0.2, 0.1)
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

    world.push_without_box(BVHNode::build(objects));

    world
}

pub fn random_scene_no_bvh() -> HittableList {
    let mut world = HittableList::default();
    let mut rng = thread_rng();

    world.push(Sphere{
        center: vec(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(Lambertian {
            albedo: vec(0.5, 0.5, 0.5),
        }),
    });


    for a in -41..41 {
        for b in -41..41 {
            let choose_mat = rng.gen::<f32>();
            let center = Vector3::new(a as f32 + 0.5 * rng.gen::<f32>(), 0.2, b as f32 + 0.5 * rng.gen::<f32>());
            if choose_mat < 0.8 {
                // diffuse
                let albedo = random_vec().component_mul(&random_vec());
                world.push(Sphere{
                    center, 
                    radius: 0.2, 
                    material: Arc::new(Lambertian{albedo})});
            } else if choose_mat < 0.95 {
                // metal
                let albedo = random_vec_range(0.5, 1.0);
                let fuzz = rng.gen_range(0.0, 0.5);
                world.push(Sphere{
                    center, 
                    radius: 0.2, 
                    material: Arc::new(Metal{albedo, fuzz})})
            } else {
                // glass
                let hsl_color = HSL {h: rng.gen_range(0.0, 360.0), s: 1.0, l: 0.95};
                let rgb_color = hsl_color.to_rgb();
                let color = Vector3::new(rgb_color.0 as f32 / 255.0 , rgb_color.1 as f32 / 255.0 , rgb_color.2 as f32 / 255.0 );
                world.push(Sphere{
                    center, 
                    radius: 0.2, 
                    material: Arc::new(Dielectric { 
                        ref_idx: 1.5, 
                        reflection_color: color, 
                        refraction_color: color }),
                    }
                )
            }
        }
    }

    world.push(Sphere {
        center: vec(-0.4, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Dielectric {
            ref_idx: 1.5, 
            reflection_color: vec(1.0, 1.0, 1.0),
            refraction_color: vec(1.0, 1.0, 1.0),
        })
    });
    world.push(Sphere {
        center: vec(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Lambertian {
            albedo: vec(0.4, 0.2, 0.1)
        })
    });
    world.push(Sphere {
        center: vec(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Metal {
            albedo: vec(0.7, 0.6, 0.5),
            fuzz: 0.0
        })
    });

    world
}
