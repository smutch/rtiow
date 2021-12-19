use std::f32::consts::PI;

use nalgebra_glm::Vec3;
use palette::LinSrgb;

use crate::{
    hittable::{HitList, HitRecord, Hittable},
    ray::Ray,
};

pub enum Light {
    Point {
        position: Vec3,
        color: LinSrgb,
        luminosity: f32,
    },
}

impl Light {
    pub fn intensity(&self, hitrecord: &HitRecord, world: &HitList, tmin: f32) -> LinSrgb {
        match *self {
            Light::Point {
                position,
                color,
                luminosity,
            } => {
                let shadow_ray = Ray {
                    origin: hitrecord.pos,
                    direction: position - hitrecord.pos,
                };
                let shadow_ray_dist = shadow_ray.norm();
                let shadow_ray_normal = shadow_ray.direction / shadow_ray_dist;
                let visible = world.hit(&shadow_ray, tmin, shadow_ray_dist).is_none() as u32 as f32;
                color * visible * shadow_ray_normal.dot(&hitrecord.normal).max(0.0) * luminosity
                    / (4.0 * PI * shadow_ray_dist * shadow_ray_dist)
            }
        }
    }
}
