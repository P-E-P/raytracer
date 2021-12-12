use hit::{Hit, Hittable};
use ray::*;
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
            let pixel_color = ray_color(r);

            println!("{}", colorize(pixel_color));
        }
    }
}

fn ray_color(ray: Ray) -> Color {
    let t = hit_sphere(point!(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let n = unit_vector(ray.at(t) - vec3!(0.0, 0.0, -1.0));
        return 0.5 * color!(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
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

fn hit_sphere(center: Point3, radius: f64, ray: Ray) -> f64 {
    let oc = ray.origin() - center;
    let a = ray.direction().length_squared();
    let half_b = dot(oc, ray.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

impl Hittable for Vec<Box<dyn Hittable>> {
    fn hit(&self, ray: Ray, range: RangeInclusive<f64>) -> Option<Hit> {
        let mut hit_anything = false;
        let mut closest_so_far = *range.end();
        let mut result = None;

        for object in self.iter() {
            if let Some(hit) = object.hit(ray, *range.start()..=closest_so_far) {
                hit_anything = true;
                closest_so_far = hit.t;
                result = Some(hit);
            }
        }

        result
    }
}
