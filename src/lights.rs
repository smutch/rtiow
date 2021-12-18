use nalgebra_glm::Vec3;
use palette::LinSrgb;

use crate::{
    hittable::{HitRecord, Hittable},
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
    pub fn get_position(&self) -> Vec3 {
        match *self {
            Light::Point {
                position,
                color: _,
                luminosity: _,
            } => position,
        }
    }

    pub fn get_color(&self) -> LinSrgb {
        match *self {
            Light::Point {
                position: _,
                color,
                luminosity: _,
            } => color,
        }
    }
}
