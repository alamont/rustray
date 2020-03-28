mod ray;

#[macro_use]
extern crate slice_as_array;

use crate::ray::Ray;
use image::{ImageBuffer, Pixel, Rgb, RgbImage};
use itertools::izip;
use nalgebra::Vector3;
use rayon::prelude::*;

fn hit_sphere(center: Vector3<f32>, radius: f32, ray: &Ray) -> bool {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(&ray.direction());
    let b = 2.0 * oc.dot(&ray.direction());
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    return discriminant > 0.0;
}

fn color(ray: &Ray) -> Vector3<f32> {
    if hit_sphere(Vector3::new(0.0, 0.0, -1.0), 0.5, &ray) {
        Vector3::new(1.0, 0.0, 0.0)
    } else {
        let unit_direction: Vector3<f32> = ray.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let nx: u32 = 200;
    let ny: u32 = 100;

    let mut imgbuf: image::RgbImage = image::ImageBuffer::new(nx, ny);

    let origin = Vector3::new(0.0, 0.0, 0.0);
    let lower_left_corner = Vector3::new(-2.0, -1.0, -1.0);
    let horizontal = Vector3::new(4.0, 0.0, 0.0);
    let vertical = Vector3::new(0.0, 2.0, 0.0);

    let image = (0..ny)
        .into_par_iter()
        .rev()
        .map(|y| {
            (0..nx)
                .map(|x| {
                    let u = x as f32 / nx as f32;
                    let v = (ny as f32 - y as f32) / ny as f32;

                    let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
                    color(&ray)
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
