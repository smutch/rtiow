#![allow(dead_code)]
use std::ops::Mul;

use indicatif::ProgressBar;
use nalgebra_glm::{vec3, Vec3};
use palette::{LinSrgb, Srgb};
use rand::{prelude::ThreadRng, Rng};

mod hittable;
mod materials;
mod ray;
use crate::hittable::*;
use crate::materials::Material;
use crate::ray::*;

struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    fn new(aspect: f32) -> Self {
        let viewport_height = 2.0;
        let viewport_width = aspect * viewport_height;
        let focal_length = 1.0;
        let origin = vec3(0.0, 0.0, 0.0);
        let horizontal = vec3(viewport_width, 0.0, 0.0);
        let vertical = vec3(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal * 0.5 - vertical * 0.5 - vec3(0.0, 0.0, focal_length);
        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin,
        }
    }
}

fn ray_color(ray: &Ray, world: &HitList, depth: u32, rng: &mut ThreadRng) -> LinSrgb {
    if depth == 0 {
        return LinSrgb::new(0.0, 0.0, 0.0);
    }

    match world.hit(ray, 0.001, f32::INFINITY) {
        None => {
            let direction = ray.normalize();
            let t = 0.5 * (direction.y + 1.0);
            LinSrgb::new(1.0, 1.0, 1.0) * (1.0 - t) + LinSrgb::new(0.5, 0.7, 1.0) * t
        }
        Some(hitrecord) => match hitrecord.material.scatter(ray, &hitrecord, rng) {
            Some(event) => ray_color(&event.ray, world, depth - 1, rng).mul(event.attenuation),
            None => LinSrgb::new(0.0, 0.0, 0.0),
        },
    }
}

fn main() -> Result<(), image::ImageError> {
    const ASPECT: f32 = 16.0 / 9.0;
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = (WIDTH as f32 / ASPECT) as u32;
    const SAMPLES: u32 = 100;
    const MAXDEPTH: u32 = 50;

    let mut world = HitList::new();

    /*
     * NOTE: The way I have it, each object holds it's own material object.
     *       Might be better to reuse materials?
     */
    world.push(Box::new(Sphere::new(
        vec3(0.0, -100.5, -1.0),
        100.0,
        Material::new_lambertian(LinSrgb::new(0.8, 0.8, 0.0)),
    )));
    world.push(Box::new(Sphere::new(
        vec3(0.0, 0.0, -1.0),
        0.5,
        Material::new_dialectric(1.5),
    )));
    world.push(Box::new(Sphere::new(
        vec3(-1.0, 0.0, -1.0),
        0.5,
        Material::new_dialectric(1.5),
    )));
    world.push(Box::new(Sphere::new(
        vec3(1.0, 0.0, -1.0),
        0.5,
        Material::new_metal(LinSrgb::new(0.8, 0.6, 0.2), 1.0),
    )));

    let camera = Camera::new(ASPECT);
    let mut framebuffer = image::RgbImage::new(WIDTH, HEIGHT);

    let mut rng = rand::thread_rng();
    let pbar = ProgressBar::new(HEIGHT as u64);
    for jj in 0..HEIGHT {
        for ii in 0..WIDTH {
            let mut pixel_color = LinSrgb::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES {
                let u = (ii as f32 + rng.gen::<f32>()) / (WIDTH - 1) as f32;
                let v = (jj as f32 + rng.gen::<f32>()) / (HEIGHT - 1) as f32;
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, MAXDEPTH, &mut rng);
            }
            pixel_color /= SAMPLES as f32;
            framebuffer.put_pixel(
                ii,
                HEIGHT - jj - 1,
                image::Rgb(Srgb::from_linear(pixel_color).into_format().into()),
            );
        }
        pbar.inc(1);
    }
    pbar.finish_with_message("render complete");

    framebuffer.save("./image.png")?;

    Ok(())
}
