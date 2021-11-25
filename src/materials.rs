use palette::LinSrgb;
use rand::prelude::ThreadRng;

use crate::{
    hittable::HitRecord,
    ray::{is_near_zero, random_in_unit_sphere, random_unit_vector, reflect, Ray},
};

pub struct ScatterEvent {
    pub attenuation: LinSrgb,
    pub ray: Ray,
}

pub enum Material {
    Lambertian { albedo: LinSrgb },
    Metal { albedo: LinSrgb, fuzz: f32 },
}

impl Material {
    pub fn new_metal(albedo: LinSrgb, fuzz: f32) -> Self {
        Self::Metal {
            albedo,
            fuzz: if fuzz <= 1.0 { fuzz } else { 1.0 },
        }
    }

    pub fn new_lambertian(albedo: LinSrgb) -> Self {
        Self::Lambertian { albedo }
    }

    pub fn scatter(
        &self,
        ray_in: &Ray,
        hitrecord: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<ScatterEvent> {
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = hitrecord.normal + random_unit_vector(rng);

                if is_near_zero(scatter_direction) {
                    scatter_direction = hitrecord.normal
                };

                let scattered = Ray {
                    origin: hitrecord.pos,
                    direction: scatter_direction,
                };
                let attenuation = *albedo;
                Some(ScatterEvent {
                    attenuation,
                    ray: scattered,
                })
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = reflect(&ray_in.direction.normalize(), &hitrecord.normal);
                let scattered = Ray {
                    origin: hitrecord.pos,
                    direction: reflected + *fuzz * random_in_unit_sphere(rng),
                };
                let attenuation = *albedo;
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
    }
}
