mod ray;
mod hitable;
mod sphere;
mod camera;

#[macro_use]
extern crate slice_as_array;

use crate::ray::Ray;
use crate::hitable::{Hitable, HitRecord, HitableList};

use image::{ImageBuffer, Pixel, Rgb, RgbImage};
use itertools::izip;
use nalgebra::Vector3;
use rayon::prelude::*;
use std::f32;
use sphere::Sphere;
use camera::Camera;
use rand::{thread_rng, Rng};


fn hit_sphere(center: Vector3<f32>, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(&ray.direction());
    let b = 2.0 * oc.dot(&ray.direction());
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn ray_color(ray: &Ray, world: &HitableList) -> Vector3<f32> {

    if let Some(hit_rec) = world.hit(ray, 0.0, f32::MAX) {
        0.5 * Vector3::new(hit_rec.normal.x + 1.0, hit_rec.normal.y + 1.0, hit_rec.normal.z + 1.0)
    } else {
        let unit_direction: Vector3<f32> = ray.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
    }

    // let center = Vector3::new(0.0, 0.0, -1.0);
    // let t =  hit_sphere(center, 0.5, &ray);
    // if t > 0.0 {
    //     // Vector3::new(1.0, 0.0, 0.0)
    //     let normal = (ray.point_at_parameter(t) - center).normalize();
    //     0.5 * Vector3::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0)
    // } 
}

fn main() {
    let nx: u32 = 200;
    let ny: u32 = 100;
    let ns = 100;

    let cam = Camera::default();

    let mut world = HitableList::default();
    world.push(Sphere{center: Vector3::new(0.0, 0.0, -1.0), radius: 0.5});
    world.push(Sphere{center: Vector3::new(0.0, -100.5, -1.0), radius: 100.0});

    let image = (0..ny).into_par_iter().rev()
        .map(|y| {
            let mut rng = thread_rng();
            (0..nx).map(|x| {
                let col: Vector3<f32> = (0..ns).map(|_|{
                    let u = (x as f32 + rng.gen::<f32>())/ nx as f32;
                    let v = (ny as f32 - (y as f32 + rng.gen::<f32>())) / ny as f32;
                    let ray = cam.get_ray(u, v);
                    ray_color(&ray, &world)
                }).sum();
                col / (ns as f32)
            }).collect::<Vec<Vector3<f32>>>()
        }).collect::<Vec<Vec<Vector3<f32>>>>();

    let mut imgbuf = image::ImageBuffer::new(nx, ny);

    for (y, row) in izip!(0..ny, image) {
        for (x, image_pix) in izip!(0..nx, row) {
            let pixel = imgbuf.get_pixel_mut(x, ny - y - 1);
            let image_pix_rgb = (255.99 * image_pix)
                .iter()
                .map(|c| *c as u8)
                .collect::<Vec<u8>>();
            *pixel = Rgb([image_pix_rgb[0], image_pix_rgb[1], image_pix_rgb[2]]);
        }
    }

    imgbuf.save("image.png").unwrap();
}
