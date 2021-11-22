#![allow(dead_code)]
use nalgebra_glm::Vec3;
use std::slice::Iter;

use crate::{materials::Material, ray::Ray};

pub struct HitRecord<'a> {
    pub pos: Vec3,
    pub normal: Vec3,
    t: f32,
    material: &'a Box<dyn Material>,
    front_face: bool,
}
impl<'b> HitRecord<'b> {
    fn new(
        ray: &Ray,
        pos: Vec3,
        outward_normal: Vec3,
        t: f32,
        material: &'b Box<dyn Material>,
    ) -> Self {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
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

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    centre: Vec3,
    radius: f32,
    material: Box<dyn Material>,
}
impl Sphere {
    pub fn new(centre: Vec3, radius: f32, material: Box<dyn Material>) -> Self {
        Sphere {
            centre,
            radius,
            material,
        }
    }
}
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.centre;
        let a = ray.norm_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.norm_squared() - self.radius * self.radius;
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
        let pos = ray.origin + ray.direction * t;
        let outward_normal = (pos - self.centre).normalize();
        Some(HitRecord::new(ray, pos, outward_normal, t, &self.material))
    }
}

type HitListElement = Box<dyn Hittable>;
pub struct HitList(Vec<HitListElement>);

// This is very cool, but we really only need to be able to iterate over and push to HitList. Good
// trick to remember for future though!
// use std::ops::{Deref, DerefMut};
/* impl Deref for HitList {
    type Target = Vec<Box<dyn Hittable>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for HitList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
} */

impl HitList {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn push(&mut self, object: HitListElement) {
        self.0.push(object);
    }

    fn iter(&self) -> Iter<HitListElement> {
        self.0.iter()
    }
}

impl Hittable for HitList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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
