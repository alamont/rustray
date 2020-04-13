use crate::ray::Ray;
use crate::vec::{deg_to_rad, random_unit_in_disk};

use nalgebra::Vector3;

pub enum ApertureShape {
    Circle,
    Hexagon
}

pub struct Camera {
    pub origin: Vector3<f32>,
    pub lower_left_corner: Vector3<f32>,
    pub horizontal: Vector3<f32>,
    pub vertical: Vector3<f32>,
    u: Vector3<f32>,
    v: Vector3<f32>,
    w: Vector3<f32>,
    pub lens_radius: f32,
    pub aperture_shape: ApertureShape
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
        use ApertureShape::*;
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
            u, v, w, lens_radius,
            aperture_shape: Circle
        }
    }
    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        use ApertureShape::*;
        let rd = match &self.aperture_shape {
            Circle => self.lens_radius * random_unit_in_disk(),
            Hexagon => {
                loop {
                    let p = self.lens_radius * random_unit_in_disk();
                    if inside_hexagon(self.lens_radius * 2.0, p.x, p.y) {
                        break p;
                    }
                }
            }
        };
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

const A: f32 = 0.25 * 1.73205080757;

fn inside_hexagon(d: f32, x: f32, y: f32) -> bool {
    let dx = x.abs() / d;
    let dy = y.abs() / d;
    (dy <= A) && (A*dx + 0.25*dy <= 0.5*A)
}