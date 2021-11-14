use indicatif::ProgressBar;
use nalgebra_glm::{vec3, Vec3};
use palette::Srgb;

fn ray_color(r: &Vec3, origin: &Vec3) -> Srgb {
    let sphere = Sphere::new(vec3(0.0, 0.0, -1.0), 0.5);
    let components = match sphere.hit(r, origin, 0.0, 9999.9) {
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
    Srgb::new(components[0], components[1], components[2])
}

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
}

struct HitRecord {
    p: Vec3,
    normal: Vec3,
    t: f32
}

trait Hittable {
    fn hit(&self, ray: &Vec3, origin: &Vec3, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

struct Sphere {
    centre: Vec3,
    radius: f32
}
impl Sphere {
    fn new(centre: Vec3, radius: f32) -> Self {
        Sphere{centre, radius}
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
        let p = origin + ray*t;
        let normal = (p - self.centre).normalize();
        Some(HitRecord{p, normal, t})
    }
} 


fn main() -> Result<(), image::ImageError> {
    const ASPECT: f32 = 16.0 / 9.0;
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = (WIDTH as f32 / ASPECT) as u32;

    let camera = Camera::new(ASPECT);
    let mut framebuffer = image::RgbImage::new(WIDTH, HEIGHT);

    let pbar = ProgressBar::new(HEIGHT as u64);
    for jj in 0..HEIGHT {
        for ii in 0..WIDTH {
            let u = ii as f32 / (WIDTH - 1) as f32;
            let v = jj as f32 / (HEIGHT - 1) as f32;
            let ray = camera.lower_left_corner + u * camera.horizontal + v * camera.vertical
                - camera.origin;
            let color = ray_color(&ray, &camera.origin).into_format().into();
            framebuffer.put_pixel(ii, HEIGHT - jj - 1, image::Rgb(color));
        }
        pbar.inc(1);
    }
    pbar.finish_with_message("render complete");

    framebuffer.save("./image.png")?;

    Ok(())
}
