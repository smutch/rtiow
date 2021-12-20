#![allow(dead_code)]
use nalgebra_glm::Vec3;
use std::slice::Iter;

use crate::{materials::Material, ray::Ray};

pub struct HitRecord<'a> {
    pub pos: Vec3,
    pub normal: Vec3,
    t: f32,
    pub material: &'a Material,
    pub front_face: bool,
}
impl<'b> HitRecord<'b> {
    fn new(ray: &Ray, pos: Vec3, outward_normal: Vec3, t: f32, material: &'b Material) -> Self {
        let front_face = ray.direction.dot(&outward_normal) <= 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord::<'b> {
            pos,
            normal,
            material,
            t,
            front_face,
        }
    }
}

// pub trait Hittable: HittableClone + Send + Sync {
//     fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
// }

pub enum Hittable {
    Sphere {
        centre: Vec3,
        radius: f32,
        material: Material,
    },
}

impl Hittable {
    pub fn new_sphere(centre: Vec3, radius: f32, material: Material) -> Hittable {
        Self::Sphere {
            centre,
            radius,
            material,
        }
    }
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match *self {
            Hittable::Sphere {
                centre,
                radius,
                ref material,
            } => {
                let oc = ray.origin - centre;
                let a = ray.norm_squared();
                let half_b = oc.dot(&ray.direction);
                let c = oc.norm_squared() - radius * radius;
                let discriminant = half_b * half_b - a * c;

                if discriminant < 0.0 {
                    return None;
                }

                // Find the nearest root that lies in an acceptible range (t_min < t < t_max)
                let sqrtd = discriminant.sqrt();
                let mut root = (-half_b - sqrtd) / a;
                if root < t_min || t_max < root {
                    root = (-half_b + sqrtd) / a;
                    if root < t_min || t_max < root {
                        return None;
                    }
                }

                let t = root;
                let pos = ray.at(t);
                let outward_normal = (pos - centre) / radius;
                Some(HitRecord::new(ray, pos, outward_normal, t, material))
            }
        }
    }
}

pub struct HitList(Vec<Hittable>);

impl HitList {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn push(&mut self, object: Hittable) {
        self.0.push(object);
    }

    fn iter(&self) -> Iter<Hittable> {
        self.0.iter()
    }

    pub fn trace(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_t = t_max;
        let mut closest_hitrecord: Option<HitRecord> = None;

        for object in self.iter() {
            if let Some(hitrecord) = object.hit(ray, t_min, closest_t) {
                closest_t = hitrecord.t;
                closest_hitrecord = Some(hitrecord);
            }
        }

        closest_hitrecord
    }
}
