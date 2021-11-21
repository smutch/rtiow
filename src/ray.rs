use nalgebra_glm::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t*self.direction
    }

    pub fn normalize(&self) -> Vec3 {
        self.direction.normalize()
    }

    pub fn norm_squared(&self) -> f32 {
        self.direction.norm_squared()
    }
}
