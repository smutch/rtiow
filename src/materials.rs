use palette::LinSrgb;
use rand::prelude::ThreadRng;

use crate::{
    hittable::HitRecord,
    ray::{is_near_zero, random_unit_vector, reflect, Ray},
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

pub struct Metal {
    pub albedo: LinSrgb,
}
impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        hitrecord: &HitRecord,
        _rng: &mut ThreadRng,
    ) -> Option<ScatterEvent> {
        let reflected = reflect(&ray_in.direction.normalize(), &hitrecord.normal);
        let scattered = Ray {
            origin: hitrecord.pos,
            direction: reflected,
        };
        let attenuation = self.albedo;
        if scattered.direction.dot(&hitrecord.normal) > 0.0 {
            Some(ScatterEvent {
                attenuation,
                ray: scattered,
            })
        } else {
            None
        }
    }
}
