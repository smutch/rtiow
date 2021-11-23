use palette::LinSrgb;
use rand::prelude::ThreadRng;

use crate::{
    hittable::HitRecord,
    ray::{is_near_zero, random_unit_vector, Ray},
};

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        hitrecord: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<ScatterEvent>;
}

pub struct ScatterEvent {
    pub attenuation: LinSrgb,
    pub ray: Ray,
}

pub struct Lambertian {
    pub albedo: LinSrgb,
}
impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray,
        hitrecord: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<ScatterEvent> {
        let mut scatter_direction = hitrecord.normal + random_unit_vector(rng);

        if is_near_zero(scatter_direction) {
            scatter_direction = hitrecord.normal
        };

        let scattered = Ray {
            origin: hitrecord.pos,
            direction: scatter_direction,
        };
        let attenuation = self.albedo;
        Some(ScatterEvent {
            attenuation,
            ray: scattered,
        })
    }
}
