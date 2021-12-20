use nalgebra_glm::vec3;
use palette::LinSrgb;
use rand::{distributions::Uniform, prelude::Distribution};

use crate::{
    hittable::{HitList, Hittable},
    lights::Light,
    materials::Material,
    Camera, Scene,
};

pub fn scene() -> Scene {
    let mut world = HitList::new();

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

    // image settings
    let aspect: f32 = 3.0 / 2.0;
    let width: u32 = 200;

    // render settings
    let nframes: u32 = 4;
    let samples = 400u32 / nframes;
    let maxdepth: u32 = 50;

    Scene {
        aspect,
        width,
        height: (width as f32 / aspect) as u32,
        nframes,
        samples: samples / nframes,
        maxdepth,
        hitlist: world,
        lights,
    }
}

pub fn camera(scene: &Scene) -> Camera {
    let fofdegs: f32 = 20.0;
    let lookfrom = vec3(13., 2., 3.);
    let lookat = vec3(0., 0., 0.);
    let viewup = vec3(0., 1., 0.);
    let aperture: f32 = 0.1;
    let focus_dist: f32 = 10.0;

    Camera::new(
        lookfrom,
        lookat,
        viewup,
        fofdegs,
        scene.aspect,
        aperture,
        focus_dist,
    )
}
