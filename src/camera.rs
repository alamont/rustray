use crate::ray::Ray;
use crate::vec::deg_to_rad;

use nalgebra::Vector3;

pub struct Camera {
    pub origin: Vector3<f32>,
    pub lower_left_corner: Vector3<f32>,
    pub horizontal: Vector3<f32>,
    pub vertical: Vector3<f32>,
}

impl Camera {
    pub fn new(
        origin: Vector3<f32>,
        lookat: Vector3<f32>,
        vup: Vector3<f32>,
        vfov: f32,
        aspect: f32,
    ) -> Camera {
        let theta = deg_to_rad(vfov);
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (origin - lookat).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let lower_left_corner = origin - half_width * u - half_height * v - w;
        let horizontal = 2.0 * half_width * u;
        let vertical = 2.0 * half_height * v;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            origin: Vector3::new(0.0, 0.0, 0.0),
            lower_left_corner: Vector3::new(-2.0, -1.0, -1.0),
            horizontal: Vector3::new(4.0, 0.0, 0.0),
            vertical: Vector3::new(0.0, 2.0, 0.0),
        }
    }
}
