use aabb::Aabb;
use hit::{Hit, Hittable};
use camera::Camera;
use scene::random_scene;
use rand::distributions::{Distribution, Uniform};
use render::{RenderSettings, render};
use ray::*;
use vec3::*;

mod hit;
mod moving_sphere;
mod ray;
mod sphere;
#[macro_use]
mod vec3;
mod aabb;
mod bvh;
mod camera;
mod material;
mod utils;
mod render;
mod scene;


fn main() {
    let settings = RenderSettings::new(200, 16.0 / 9.0);

    // World
    let world: Vec<Box<dyn Hittable>> = random_scene();
    let mut rng = rand::thread_rng();
    let dist = Uniform::from(0.0..=1.0);

    // Camera
    let look_from = point!(13.0, 2.0, 3.0);
    let look_at = point!(0.0, 0.0, 0.0);

    let cam = camera::Builder::new(look_from, look_at, 20.0, settings.aspect_ratio, 0.1, 10.0)
        .timed(0.0, 1.0)
        .build();

    let pixels = render(&world, &cam, settings);

    println!("P3\n{} {}\n255\n", settings.image_width, settings.image_height);

    for j in (0..settings.image_height).rev() {
        eprintln!("Scanline remaining: {}", j);
        for i in 0..settings.image_width {
            let mut pixel_color = Color::default();

            for _ in 0..settings.sample_per_pixel {
                let u = (i as f64 + dist.sample(&mut rng)) / (settings.image_width - 1) as f64;
                let v = (j as f64 + dist.sample(&mut rng)) / (settings.image_height - 1) as f64;
                let ray = cam.get_ray(u, v);
                pixel_color += ray_color(ray, &world, settings.max_depth);
            }

            println!("{}", colorize(pixel_color, settings.sample_per_pixel));
        }
    }
}

fn ray_color(ray: Ray, world: &impl Hittable, depth: usize) -> Color {
    if depth == 0 {
        return Color::default();
    }

    if let Some(hit) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some((scattered, attenuation)) = hit.material.scatter(&ray, &hit) {
            return attenuation * ray_color(scattered, world, depth - 1);
        }
        return Color::default();
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
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut closest_so_far = t_max;
        let mut result = None;

        for object in self.iter() {
            if let Some(hit) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                result = Some(hit);
            }
        }

        result
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if self.is_empty() {
            return None;
        }

        let mut result: Option<Aabb> = None;

        for object in self {
            match object.bounding_box(time0, time1) {
                Some(current) => match result {
                    Some(previous) => {
                        result = Some(Aabb::surrounding(&previous, &current));
                    }
                    None => {
                        result = Some(current);
                    }
                },
                None => return None,
            }
        }

        result
    }
}
