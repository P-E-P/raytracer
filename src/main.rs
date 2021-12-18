use hit::{Hit, Hittable};
use ray::*;
use sphere::Sphere;
use std::ops::RangeInclusive;
use vec3::*;

mod hit;
mod ray;
mod sphere;
#[macro_use]
mod vec3;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    // World
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();
    world.push(Box::new(Sphere::new(point!(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(point!(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = point!(0.0, 0.0, 0.0);
    let horizontal = vec3!(viewport_width, 0.0, 0.0);
    let vertical = vec3!(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - vec3!(0.0, 0.0, focal_length);

    println!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("Scanline remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(r, &world);

            println!("{}", colorize(pixel_color));
        }
    }
}

fn ray_color(ray: Ray, world: &impl Hittable) -> Color {
    if let Some(hit) = world.hit(ray, 0.0..=f64::INFINITY) {
        return 0.5 * (hit.normal + color!(1.0, 1.0, 1.0));
    }
    let unit_direction = unit_vector(ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * color!(1.0, 1.0, 1.0) + t * color!(0.5, 0.7, 1.0)
}

fn colorize(color: Color) -> String {
    format!(
        "{} {} {}",
        (255.999 * color.x()) as usize,
        (255.999 * color.y()) as usize,
        (255.999 * color.z()) as usize
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
