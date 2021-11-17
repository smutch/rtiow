#![allow(dead_code)]
use nalgebra_glm::Vec3;
use std::slice::Iter;

pub struct HitRecord {
    pos: Vec3,
    pub normal: Vec3,
    t: f32,
    front_face: bool,
}
impl HitRecord {
    fn new(ray: &Vec3, pos: Vec3, outward_normal: Vec3, t: f32) -> Self {
        let front_face = ray.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            pos,
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Vec3, origin: &Vec3, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    centre: Vec3,
    radius: f32,
}
impl Sphere {
    pub fn new(centre: Vec3, radius: f32) -> Self {
        Sphere { centre, radius }
    }
}
impl Hittable for Sphere {
    fn hit(&self, ray: &Vec3, origin: &Vec3, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = origin - self.centre;
        let a = ray.norm_squared();
        let half_b = oc.dot(ray);
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
        let pos = origin + ray * t;
        let outward_normal = (pos - self.centre).normalize();
        Some(HitRecord::new(ray, pos, outward_normal, t))
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
    fn hit(&self, ray: &Vec3, origin: &Vec3, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_t = t_max;
        let mut closest_hitrecord: Option<HitRecord> = None;

        for object in self.iter() {
            if let Some(hitrecord) = object.hit(ray, origin, t_min, closest_t) {
                closest_t = hitrecord.t;
                closest_hitrecord = Some(hitrecord);
            }
        }

        closest_hitrecord
    }
}