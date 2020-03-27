use nalgebra::Vector3;

#[derive(Debug)]
pub struct Ray {
    a: Vector3<f32>,
    b: Vector3<f32>,
}

impl Ray {
    pub fn new(a: Vector3<f32>, b: Vector3<f32>) -> Self {
        return Ray { a, b };
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
}
