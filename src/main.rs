// #![allow(dead_code, unused_imports)]
use std::thread;

use indicatif::{MultiProgress, ProgressBar};
use nalgebra_glm::{vec3, Vec3};
use palette::{LinSrgb, Srgb};
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rand::{prelude::ThreadRng, Rng};
use rayon::prelude::*;

mod hittable;
mod lights;
mod materials;
mod ray;
use crate::hittable::*;
use crate::lights::Light;
use crate::materials::Material;
use crate::ray::*;

struct Camera {
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
    // image settings
    const ASPECT: f32 = 3.0 / 2.0;
    const WIDTH: u32 = 1200;
    const HEIGHT: u32 = (WIDTH as f32 / ASPECT) as u32;

    // camera settings
    const FOFDEGS: f32 = 20.0;
    const LOOKFROM: Vec3 = Vec3::new(13., 2., 3.);
    const LOOKAT: Vec3 = Vec3::new(0., 0., 0.);
    const VIEWUP: Vec3 = Vec3::new(0., 1., 0.);
    const APERTURE: f32 = 0.1;
    const FOCUS_DIST: f32 = 10.0;

    // render settings
    const NFRAMES: u32 = 8;
    const SAMPLES: u32 = 1000 / NFRAMES;
    const MAXDEPTH: u32 = 50;

    let mut world = HitList::new();

    /*
     * NOTE: The way I have it, each object holds it's own material object.
     *       Might be better to reuse materials?
     */

    // ground
    world.push(Hittable::new_sphere(
        vec3(0.0, -1000., 0.0),
        1000.0,
        Material::new_lambertian(LinSrgb::new(0.5, 0.5, 0.5)),
    ));

    let mut rng = rand::thread_rng();
    let distrib = Uniform::new(0f32, 1f32);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = distrib.sample(&mut rng);
            let center = vec3(
                a as f32 + 0.9 * distrib.sample(&mut rng),
                0.2,
                b as f32 + 0.9 * distrib.sample(&mut rng),
            );

            if (center - vec3(4.0, 0.2, 0.0)).norm() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = LinSrgb::new(
                        distrib.sample(&mut rng),
                        distrib.sample(&mut rng),
                        distrib.sample(&mut rng),
                    ) * LinSrgb::new(
                        distrib.sample(&mut rng),
                        distrib.sample(&mut rng),
                        distrib.sample(&mut rng),
                    );
                    world.push(Hittable::new_sphere(
                        center,
                        0.2,
                        Material::new_lambertian(albedo),
                    ));
                } else if choose_mat < 0.95 {
                    // metal
                    let distrib = Uniform::new(0.5f32, 1f32);
                    let albedo = LinSrgb::new(
                        distrib.sample(&mut rng),
                        distrib.sample(&mut rng),
                        distrib.sample(&mut rng),
                    );
                    let fuzz = distrib.sample(&mut rng);
                    world.push(Hittable::new_sphere(
                        center,
                        0.2,
                        Material::new_metal(albedo, fuzz),
                    ));
                } else {
                    // glass
                    world.push(Hittable::new_sphere(
                        center,
                        0.2,
                        Material::new_dialectric(1.5),
                    ));
                }
            }
        }
    }

    world.push(Hittable::new_sphere(
        vec3(-4., 1., 0.),
        1.0,
        Material::new_lambertian(LinSrgb::new(0.4, 0.2, 0.1)),
    ));

    world.push(Hittable::new_sphere(
        vec3(0., 1., 0.),
        1.0,
        Material::new_dialectric(1.5),
    ));

    world.push(Hittable::new_sphere(
        vec3(4., 1., 0.),
        1.0,
        Material::new_metal(LinSrgb::new(0.7, 0.6, 0.5), 0.0),
    ));

    let lights = vec![Light::Point {
        position: vec3(1.0, 5.0, 0.0),
        color: LinSrgb::new(1.0, 1.0, 1.0),
        luminosity: 100.0,
    }];

    let camera = Camera::new(
        LOOKFROM, LOOKAT, VIEWUP, FOFDEGS, ASPECT, APERTURE, FOCUS_DIST,
    );

    let multiprogress = MultiProgress::new();
    let mut pbars = Vec::new();
    for _ in 0..NFRAMES {
        pbars.push(multiprogress.add(ProgressBar::new(WIDTH as u64)));
    }

    thread::spawn(move || multiprogress.join().unwrap());

    let mut framebuffer = (0..NFRAMES)
        .into_par_iter()
        .map(|iframe| {
            let mut framebuffer = Vec::with_capacity((WIDTH * HEIGHT) as usize);
            let mut rng = rand::thread_rng();
            for ii in 0..WIDTH {
                for jj in 0..HEIGHT {
                    let mut pixel_color = LinSrgb::new(0.0, 0.0, 0.0);
                    for _ in 0..SAMPLES {
                        let u = (ii as f32 + rng.gen::<f32>()) / (WIDTH - 1) as f32;
                        let v = (jj as f32 + rng.gen::<f32>()) / (HEIGHT - 1) as f32;
                        let ray = camera.get_ray(u, v, &mut rng);
                        pixel_color += ray_color(&ray, &world, &lights, MAXDEPTH, &mut rng);
                    }
                    pixel_color /= SAMPLES as f32;
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
        *v /= NFRAMES as f32;
    }

    let mut frame = image::RgbImage::new(WIDTH, HEIGHT);

    for jj in 0..HEIGHT {
        for ii in 0..WIDTH {
            frame.put_pixel(
                ii,
                HEIGHT - jj - 1,
                image::Rgb(
                    Srgb::from_linear(framebuffer[(ii * HEIGHT + jj) as usize])
                        .into_format()
                        .into(),
                ),
            );
        }
    }

    frame.save("./image.png")?;

    Ok(())
}
