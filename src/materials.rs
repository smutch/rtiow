use palette::LinSrgb;
use rand::prelude::ThreadRng;

use crate::{hittable::HitRecord, ray::*};

pub struct ScatterEvent {
    pub attenuation: LinSrgb,
    pub ray: Ray,
}

pub enum Material {
    Lambertian { albedo: LinSrgb },
    Metal { albedo: LinSrgb, fuzz: f32 },
    Dialectric { refractive_index: f32 },
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

    pub fn new_dialectric(refractive_index: f32) -> Self {
        Self::Dialectric { refractive_index }
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

            Material::Dialectric { refractive_index } => {
                let attenuation = LinSrgb::new(1.0, 1.0, 1.0);
                let refraction_ratio = if hitrecord.front_face {
                    1.0 / *refractive_index
                } else {
                    *refractive_index
                };
                let unit_direction = ray_in.direction.normalize();

                let cos_theta = (-unit_direction).dot(&hitrecord.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let direction = if refraction_ratio * sin_theta > 1.0 {
                    reflect(&unit_direction, &hitrecord.normal)
                } else {
                    refract(&unit_direction, &hitrecord.normal, refraction_ratio)
                };

                Some(ScatterEvent {
                    attenuation,
                    ray: Ray {
                        origin: hitrecord.pos,
                        direction,
                    },
                })
            }
        }
    }
}
