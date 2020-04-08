use crate::ray::Ray;
use crate::vec::{deg_to_rad, random_unit_in_disk};

use nalgebra::Vector3;

pub struct Camera {
    pub origin: Vector3<f32>,
    pub lower_left_corner: Vector3<f32>,
    pub horizontal: Vector3<f32>,
    pub vertical: Vector3<f32>,
    u: Vector3<f32>,
    v: Vector3<f32>,
    w: Vector3<f32>,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        origin: Vector3<f32>,
        lookat: Vector3<f32>,
        vup: Vector3<f32>,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32
    ) -> Camera {
        let lens_radius = aperture / 2.0;

        let theta = deg_to_rad(vfov);
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;       

        let w = (origin - lookat).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let lower_left_corner = origin 
                              - half_width * focus_dist * u 
                              - half_height * focus_dist * v 
                              - focus_dist * w;
        let horizontal = 2.0 * half_width * focus_dist * u;
        let vertical = 2.0 * half_height * focus_dist * v;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u, v, w, lens_radius
        }
    }
    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_unit_in_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }

    pub fn get_ray_an(&self, s: f32, t: f32) -> Ray {
        let mut ray = self.get_ray(s, t);
        ray.albedo_normal_ray = true;
        ray
    }
}

impl Default for Camera {
    fn default() -> Self {
        let origin = Vector3::new(3.0, 3.0, 2.0);
        let lookat = Vector3::new(0.0, 0.0, -1.0);
        let vup = Vector3::new(0.0, 1.0, 0.0);        
        let aspect = 2.0;
        let aperture = 0.5;
        let dist_to_focus = (origin-lookat).magnitude();
        Camera::new(
            origin,
            lookat,
            vup, 20.0, aspect,
            aperture,
            dist_to_focus
        )
    }
}
