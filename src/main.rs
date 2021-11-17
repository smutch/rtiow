#![allow(dead_code)]
use indicatif::ProgressBar;
use nalgebra_glm::{vec3, Vec3};
use palette::LinSrgb;
use rand::Rng;

mod hittable;
use crate::hittable::*;

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

    fn get_ray(&self, u: f32, v: f32) -> Vec3 {
        self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin
    }
}

fn ray_color(r: &Vec3, origin: &Vec3, world: &HitList) -> LinSrgb {
    let components = match world.hit(r, origin, 0.0, 9999.9) {
        None => {
            let direction = r.normalize();
            let t = 0.5 * (direction.y + 1.0);
            (1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0)
        }
        Some(hitrecord) => {
            let n = hitrecord.normal;
            0.5 * vec3(n.x + 1.0, n.y + 1.0, n.z + 1.0)
        }
    };
    LinSrgb::new(components[0], components[1], components[2])
}

fn main() -> Result<(), image::ImageError> {
    const ASPECT: f32 = 16.0 / 9.0;
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = (WIDTH as f32 / ASPECT) as u32;
    const SAMPLES: u32 = 100;

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
                pixel_color += ray_color(&ray, &camera.origin, &world);
            }
            pixel_color /= SAMPLES as f32;
            framebuffer.put_pixel(ii, HEIGHT - jj - 1, image::Rgb(pixel_color.into_format().into()));
        }
        pbar.inc(1);
    }
    pbar.finish_with_message("render complete");

    framebuffer.save("./image.png")?;

    Ok(())
}
