mod ray;

use crate::ray::Ray;
use image;
use nalgebra::Vector3;

fn color(ray: &Ray) -> Vector3<f32>{
    let unit_direction: Vector3<f32> = ray.direction().normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);
}

fn main() {
    let nx = 200;
    let ny = 100;

    let mut imgbuf: image::RgbImage = image::ImageBuffer::new(nx, ny);

    let origin = Vector3::new(0.0, 0.0, 0.0);
    let lower_left_corner = Vector3::new(-2.0, -1.0, -1.0);
    let horizontal = Vector3::new(4.0, 0.0, 0.0);
    let vertical = Vector3::new(0.0, 2.0, 0.0);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {

        let u = x as f32 / nx as f32;
        let v = (ny as f32 - y as f32) / ny as f32;

        let ray = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);
        let color = 255.0 * color(&ray);

        *pixel = image::Rgb([color.x as u8, color.y as u8, color.z as u8]);
    }

    imgbuf.save("chap3.png").unwrap();
}