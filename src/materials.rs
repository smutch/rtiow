use palette::LinSrgb;
use rand::prelude::ThreadRng;

use crate::{
    hittable::HitRecord,
    ray::{is_near_zero, random_in_unit_sphere, random_unit_vector, reflect, Ray},
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
    albedo: LinSrgb,
}
impl Lambertian {
    pub fn new(albedo: LinSrgb) -> Self {
        Self { albedo }
    }
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
    albedo: LinSrgb,
    fuzz: f32,
}
impl Metal {
    pub fn new(albedo: LinSrgb, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: if fuzz <= 1.0 { fuzz } else { 1.0 },
        }
    }
}
impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        hitrecord: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<ScatterEvent> {
        let reflected = reflect(&ray_in.direction.normalize(), &hitrecord.normal);
        let scattered = Ray {
            origin: hitrecord.pos,
            direction: reflected + self.fuzz * random_in_unit_sphere(rng),
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
