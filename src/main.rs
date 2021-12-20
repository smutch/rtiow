// #![allow(dead_code, unused_imports)]
use std::thread;

use indicatif::{MultiProgress, ProgressBar};
use nalgebra_glm::Vec3;
use palette::{LinSrgb, Srgb};
use rand::{prelude::ThreadRng, Rng};
use rayon::prelude::*;

mod default_scene;
mod hittable;
mod lights;
mod materials;
mod ray;
use crate::hittable::*;
use crate::lights::Light;
use crate::ray::*;

pub struct Scene {
    // image settings
    aspect: f32,
    width: u32,
    height: u32,

    // render settings
    nframes: u32,
    samples: u32,
    maxdepth: u32,

    // content
    hitlist: HitList,
    lights: Vec<Light>,
}

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    u: Vec3,
    v: Vec3,
    _w: Vec3,
    lens_radius: f32,
}

impl Camera {
    fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        viewup: Vec3,
        fof: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let theta = fof.to_radians();
        let h = (theta * 0.5).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect * viewport_height;

        let w = (lookfrom - lookat).normalize();
        let u = viewup.cross(&w).normalize();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal * 0.5 - vertical * 0.5 - focus_dist * w;

        let lens_radius = aperture * 0.5;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            _w: w,
            lens_radius,
        }
    }

    fn get_ray(&self, s: f32, t: f32, rng: &mut ThreadRng) -> Ray {
        let random_offset = self.lens_radius * random_in_unit_disk(rng);
        let transformed_offset = self.u * random_offset.x + self.v * random_offset.y;
        Ray {
            origin: self.origin + transformed_offset,
            direction: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - transformed_offset,
        }
    }
}

fn ray_color(
    ray: &Ray,
    world: &HitList,
    lights: &[Light],
    depth: u32,
    rng: &mut ThreadRng,
) -> LinSrgb {
    const TMIN: f32 = 0.005;

    if depth == 0 {
        return LinSrgb::new(0.0, 0.0, 0.0);
    }

    match world.trace(ray, TMIN, f32::INFINITY) {
        None => {
            let direction = ray.normalize();
            let t = 0.5 * (direction.y + 1.0);
            (LinSrgb::new(1.0, 1.0, 1.0) * (1.0 - t) + LinSrgb::new(0.5, 0.7, 1.0) * t) * 0.5
        }
        Some(hitrecord) => {
            let intensity = lights
                .iter()
                .fold(LinSrgb::new(0.0, 0.0, 0.0), |acc, light| {
                    acc + light.intensity(&hitrecord, world, TMIN)
                });
            match hitrecord.material.scatter(ray, &hitrecord, rng) {
                Some(event) => {
                    /*
                     * TODO: I'm not sure this is right...
                     */
                    (ray_color(&event.ray, world, lights, depth - 1, rng) + intensity)
                        * event.attenuation
                }
                None => LinSrgb::new(0.0, 0.0, 0.0),
            }
        }
    }
}

fn main() -> Result<(), image::ImageError> {
    let scene = default_scene::scene();
    let camera = default_scene::camera(&scene);

    let multiprogress = MultiProgress::new();
    let mut pbars = Vec::new();
    for _ in 0..scene.nframes {
        pbars.push(multiprogress.add(ProgressBar::new(scene.width as u64)));
    }

    thread::spawn(move || multiprogress.join().unwrap());

    let mut framebuffer = (0..scene.nframes)
        .into_par_iter()
        .map(|iframe| {
            let mut framebuffer = Vec::with_capacity((scene.width * scene.height) as usize);
            let mut rng = rand::thread_rng();
            for ii in 0..scene.width {
                for jj in 0..scene.height {
                    let mut pixel_color = LinSrgb::new(0.0, 0.0, 0.0);
                    for _ in 0..scene.samples {
                        let u = (ii as f32 + rng.gen::<f32>()) / (scene.width - 1) as f32;
                        let v = (jj as f32 + rng.gen::<f32>()) / (scene.height - 1) as f32;
                        let ray = camera.get_ray(u, v, &mut rng);
                        pixel_color += ray_color(
                            &ray,
                            &scene.hitlist,
                            &scene.lights,
                            scene.maxdepth,
                            &mut rng,
                        );
                    }
                    pixel_color /= scene.samples as f32;
                    framebuffer.push(pixel_color);
                }
                if ii % 10 == 0 {
                    pbars[iframe as usize].inc(10);
                }
            }
            pbars[iframe as usize].finish_with_message("done");
            framebuffer
        })
        .reduce_with(|accum, x| accum.iter().zip(x.iter()).map(|(&a, &b)| a + b).collect())
        .unwrap();

    for v in framebuffer.iter_mut() {
        *v /= scene.nframes as f32;
    }

    let mut frame = image::RgbImage::new(scene.width, scene.height);

    for jj in 0..scene.height {
        for ii in 0..scene.width {
            frame.put_pixel(
                ii,
                scene.height - jj - 1,
                image::Rgb(
                    Srgb::from_linear(framebuffer[(ii * scene.height + jj) as usize])
                        .into_format()
                        .into(),
                ),
            );
        }
    }

    frame.save("./image.png")?;

    Ok(())
}
