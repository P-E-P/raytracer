use camera::Camera;
use hit::{Hit, Hittable};
use material::dielectric::Dielectric;
use material::lambertian::Lambertian;
use material::metal::Metal;
use rand::distributions::{Distribution, Uniform};
use ray::*;
use sphere::Sphere;
use std::ops::RangeInclusive;
use std::sync::Arc;
use utils::random;
use vec3::*;

mod hit;
mod ray;
mod sphere;
#[macro_use]
mod vec3;
mod camera;
mod material;
mod utils;

fn random_scene() -> Vec<Box<dyn Hittable>> {
    let mut world: Vec<Box<dyn Hittable>> = vec![];
    let material_ground = Arc::new(Lambertian::new(color!(0.5, 0.5, 0.5)));
    world.push(Box::new(Sphere::new(
        point!(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random(0.0..=1.0);
            let center = point!(
                a as f64 + 0.9 * random(0.0..=1.0),
                0.2,
                b as f64 + 0.9 * random(0.0..=1.0)
            );
            if (center - point!(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.push(Box::new(Sphere::new(center, 0.2, sphere_material.clone())));
                } else if choose_mat < 0.95 {
                    let albedo = Color::delimited(0.5..=1.0);
                    let fuzz = random(0.0..=0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.push(Box::new(Sphere::new(center, 0.2, sphere_material.clone())));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.push(Box::new(Sphere::new(center, 0.2, sphere_material.clone())));
                }
            }
        }
    }

    let material = Arc::new(Dielectric::new(1.5));
    world.push(Box::new(Sphere::new(
        point!(0.0, 1.0, 0.0),
        1.0,
        material.clone(),
    )));

    let material = Arc::new(Lambertian::new(color!(0.4, 0.2, 0.1)));
    world.push(Box::new(Sphere::new(
        point!(-4.0, 1.0, 0.0),
        1.0,
        material.clone(),
    )));

    let material = Arc::new(Metal::new(color!(0.7, 0.6, 0.5), 0.0));
    world.push(Box::new(Sphere::new(
        point!(4.0, 1.0, 0.0),
        1.0,
        material.clone(),
    )));

    world
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1080;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let sample_per_pixel = 100;
    let max_depth = 50;

    // World
    let world: Vec<Box<dyn Hittable>> = random_scene();
    let mut rng = rand::thread_rng();
    let dist = Uniform::from(0.0..=1.0);

    // Camera
    let look_from = point!(13.0, 2.0, 3.0);
    let look_at = point!(0.0, 0.0, 0.0);

    let cam = Camera::new(
        look_from,
        look_at,
        vec3!(0.0, 1.0, 0.0),
        20.0,
        16.0 / 9.0,
        0.1,
        10.0,
    );

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
        let mut scattered = Ray::new(point!(0.0, 0.0, 0.0), vec3!(0.0, 0.0, 0.0));
        let mut attenuation = color!(0.0, 0.0, 0.0);
        if hit
            .material
            .scatter(&ray, &hit, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(scattered, world, depth - 1);
        }
        return color!(0.0, 0.0, 0.0);
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
