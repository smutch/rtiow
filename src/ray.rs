use nalgebra_glm::{vec3, Vec3};
use rand::{
    distributions::Uniform,
    prelude::{Distribution, ThreadRng},
};

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn normalize(&self) -> Vec3 {
        self.direction.normalize()
    }

    pub fn norm_squared(&self) -> f32 {
        self.direction.norm_squared()
    }
}

pub fn random_unit_vector(rng: &mut ThreadRng) -> Vec3 {
    let distrib = Uniform::new(-1.0f32, 1.0f32);
    loop {
        let p = vec3(
            distrib.sample(rng),
            distrib.sample(rng),
            distrib.sample(rng),
        );
        if p.norm_squared() >= 1.0 {
            continue;
        }
        return p.normalize();
    }
}

pub fn is_near_zero(vec: Vec3) -> bool {
    const MIN: f32 = f32::MIN * 2.0;
    (vec[0].abs() < MIN) && (vec[1].abs() < MIN) && (vec[2].abs() < MIN)
}

pub fn reflect(vec: &Vec3, normal: &Vec3) -> Vec3 {
    vec - 2.0 * vec.dot(normal) * normal
}
