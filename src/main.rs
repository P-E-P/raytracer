use crate::vec3::*;
use aabb::Aabb;
use camera::Camera;
use hit::{Hit, Hittable};
use ray::Ray;
use render::{render, RenderSettings};
use scene::*;
use std::fs::File;
use std::io::BufWriter;

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
mod render;
mod scene;
mod utils;

fn main() {
    let settings = RenderSettings::new(400, 16.0 / 9.0);

    // World
    let world: Vec<Box<dyn Hittable>> = final_first();

    // Camera
    let look_from = point!(13.0, 2.0, 3.0);
    let look_at = point!(0.0, 0.0, 0.0);

    let cam = camera::Builder::new(look_from, look_at, 20.0, settings.aspect_ratio, 0.1, 10.0)
        .timed(0.0, 1.0)
        .build();

    let pixels = render(&world, &cam, &settings);

    let file = File::create("test.png").unwrap();
    let writer = BufWriter::new(file);
    let mut encoder = png::Encoder::new(
        writer,
        settings.image_width as u32,
        settings.image_height as u32,
    );
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&pixels).unwrap();
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
