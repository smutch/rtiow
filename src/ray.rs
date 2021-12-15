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
    #[inline(always)]
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }

    #[inline(always)]
    pub fn normalize(&self) -> Vec3 {
        self.direction.normalize()
    }

    #[inline(always)]
    pub fn norm_squared(&self) -> f32 {
        self.direction.norm_squared()
    }
}

pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
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
        return p;
    }
}

pub fn random_unit_vector(rng: &mut ThreadRng) -> Vec3 {
    random_in_unit_sphere(rng).normalize()
}

pub fn random_in_unit_disk(rng: &mut ThreadRng) -> Vec3 {
    let distrib = Uniform::new(-1.0f32, 1.0f32);
    loop {
        let p = vec3(distrib.sample(rng), distrib.sample(rng), 0f32);
        if p.norm_squared() >= 1.0 {
            continue;
        };
        return p;
    }
}

pub fn is_near_zero(vec: Vec3) -> bool {
    const MIN: f32 = f32::MIN * 2.0;
    (vec[0].abs() < MIN) && (vec[1].abs() < MIN) && (vec[2].abs() < MIN)
}

pub fn reflect(vec: &Vec3, normal: &Vec3) -> Vec3 {
    vec - 2.0 * vec.dot(normal) * normal
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = (-uv).dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = (1.0 - r_out_perp.norm_squared()).abs().sqrt() * -n;
    r_out_perp + r_out_parallel
}
