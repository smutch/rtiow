#![allow(dead_code)]
use indicatif::ProgressBar;
use nalgebra_glm::{vec3, Vec3};
use palette::{LinSrgb, Srgb};
use rand::{
    distributions::Uniform,
    prelude::{Distribution, ThreadRng},
    Rng,
};

mod hittable;
// mod materials;
mod ray;
use crate::hittable::*;
// use crate::materials::*;
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
        Ray{origin: self.origin, direction: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin}
    }
}

fn random_unit_vector(rng: &mut ThreadRng) -> Vec3 {
    let distrib = Uniform::new(-1.0f32, 1.0f32);
    loop {
        let p = vec3(
            distrib.sample(rng),
            distrib.sample(rng),
            distrib.sample(rng),
        );
        if p.norm_squared() >= 1.0 {
            continue;
        }
        return p.normalize();
    }
}

fn ray_color_components(
    ray: &Ray,
    world: &HitList,
    depth: u32,
    rng: &mut ThreadRng,
) -> Vec3 {
    if depth == 0 {
        return vec3(0f32, 0f32, 0f32);
    }

    match world.hit(ray, 0.001, f32::INFINITY) {
        None => {
            let direction = ray.normalize();
            let t = 0.5 * (direction.y + 1.0);
            (1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0)
        }
        Some(hitrecord) => {
            let target = hitrecord.pos + hitrecord.normal + random_unit_vector(rng);
            let new_ray = Ray{ origin: hitrecord.pos, direction: target - hitrecord.pos };
            0.5 * ray_color_components(
                &new_ray,
                world,
                depth - 1,
                rng,
            )
        }
    }
}

fn main() -> Result<(), image::ImageError> {
    const ASPECT: f32 = 16.0 / 9.0;
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = (WIDTH as f32 / ASPECT) as u32;
    const SAMPLES: u32 = 100;
    const MAXDEPTH: u32 = 50;

    let mut world = HitList::new();
    world.push(Box::new(Sphere::new(vec3(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(vec3(0.0, -100.5, -1.0), 100.0)));

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
                let color_components =
                    ray_color_components(&ray, &world, MAXDEPTH, &mut rng);
                pixel_color += LinSrgb::new(
                    color_components[0],
                    color_components[1],
                    color_components[2],
                );
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
