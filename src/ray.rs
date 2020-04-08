use nalgebra::Vector3;

#[derive(Debug)]
pub struct Ray {
    a: Vector3<f32>,
    b: Vector3<f32>,
    pub albedo_normal_ray: bool,
}

impl Ray {
    pub fn new(a: Vector3<f32>, b: Vector3<f32>) -> Self {
        return Ray { a, b, albedo_normal_ray: false };
    }

    pub fn origin(&self) -> Vector3<f32> {
        return self.a;
    }

    pub fn direction(&self) -> Vector3<f32> {
        return self.b;
    }

    pub fn point_at_parameter(&self, t: f32) -> Vector3<f32> {
        return self.a + t * self.b;
    }

    pub fn at(&self, t: f32) -> Vector3<f32> {
        self.point_at_parameter(t)
    }
}
