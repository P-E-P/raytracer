use crate::vec3::*;
use aabb::{Aabb, Interval};
use bvh::Bvh;
use camera::Camera;
use hit::{Hit, Hittable};
use ray::Ray;
use render::{render, RenderSettings};
use scene::*;
use std::fs::File;
use std::io::BufWriter;
use std::sync::Arc;

mod geometry;
mod hit;
mod ray;
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

    let mut world = HittableList::new();
    world.add(Arc::new(Bvh::new(final_first())));

    // Camera
    let look_from = point!(13.0, 2.0, 3.0);
    let look_at = point!(0.0, 0.0, 0.0);

    let cam = camera::Builder::new(look_from, look_at, 20.0, 0.1, 10.0)
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

struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
    bbox: Aabb,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: vec![],
            bbox: Aabb::default(),
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.bbox = Aabb::surrounding(&self.bbox, &object.bounding_box());
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<Hit> {
        let mut closest_so_far = ray_t.maximum;
        let mut result = None;

        for object in &self.objects {
            if let Some(hit) = object.hit(ray, Interval::new(ray_t.minimum, closest_so_far)) {
                closest_so_far = hit.t;
                result = Some(hit);
            }
        }

        result
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
