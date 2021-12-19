use camera::Camera;
use hit::{Hit, Hittable};
use rand::distributions::{Distribution, Uniform};
use ray::*;
use sphere::Sphere;
use std::ops::RangeInclusive;
use vec3::*;

mod hit;
mod ray;
mod sphere;
#[macro_use]
mod vec3;
mod camera;
mod utils;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 480;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let sample_per_pixel = 30;
    let max_depth = 10;

    // World
    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(point!(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(point!(0.0, -100.5, -1.0), 100.0)),
    ];
    let mut rng = rand::thread_rng();
    let dist = Uniform::from(0.0..=1.0);

    // Camera
    let cam = Camera::new();

    println!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("Scanline remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color = color!(0.0, 0.0, 0.0);

            for _ in 0..sample_per_pixel {
                let u = (i as f64 + dist.sample(&mut rng)) / (image_width - 1) as f64;
                let v = (j as f64 + dist.sample(&mut rng)) / (image_height - 1) as f64;
                let ray = cam.get_ray(u, v);
                pixel_color += ray_color(ray, &world, max_depth);
            }

            println!("{}", colorize(pixel_color, sample_per_pixel));
        }
    }
}

fn ray_color(ray: Ray, world: &impl Hittable, depth: usize) -> Color {
    if depth == 0 {
        return color!(0.0, 0.0, 0.0);
    }

    if let Some(hit) = world.hit(ray, 0.001..=f64::INFINITY) {
        let target = hit.p + hit.normal + Vec3::random_unit();
        return 0.5 * ray_color(Ray::new(hit.p, target - hit.p), world, depth - 1);
    }
    let unit_direction = unit_vector(ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * color!(1.0, 1.0, 1.0) + t * color!(0.5, 0.7, 1.0)
}

fn colorize(color: Color, spp: usize) -> String {
    let scale = 1.0 / spp as f64;
    let r = (color.x() * scale).sqrt();
    let g = (color.y() * scale).sqrt();
    let b = (color.z() * scale).sqrt();

    format!(
        "{} {} {}",
        (256.0 * utils::clamp(r, 0.0, 0.999)) as usize,
        (256.0 * utils::clamp(g, 0.0, 0.999)) as usize,
        (256.0 * utils::clamp(b, 0.0, 0.999)) as usize
    )
}

impl Hittable for Vec<Box<dyn Hittable>> {
    fn hit(&self, ray: Ray, range: RangeInclusive<f64>) -> Option<Hit> {
        let mut closest_so_far = *range.end();
        let mut result = None;

        for object in self.iter() {
            if let Some(hit) = object.hit(ray, *range.start()..=closest_so_far) {
                closest_so_far = hit.t;
                result = Some(hit);
            }
        }

        result
    }
}
