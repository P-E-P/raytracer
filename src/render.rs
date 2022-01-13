use crate::ray::Ray;
use crate::vec3::unit_vector;
use crate::Camera;
use crate::Color;
use crate::Hittable;
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use rand::distributions::{Distribution, Uniform};
use rayon::prelude::*;

#[derive(Copy, Clone)]
pub struct RenderSettings {
    pub aspect_ratio: f64,
    pub image_height: usize,
    pub image_width: usize,
    pub sample_per_pixel: usize,
    pub max_depth: usize,
}

impl RenderSettings {
    pub fn new(image_width: usize, aspect_ratio: f64) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as usize;
        RenderSettings {
            aspect_ratio,
            image_width,
            image_height,
            ..Default::default()
        }
    }
}

impl Default for RenderSettings {
    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let image_width = 1920;
        let image_height = (image_width as f64 / aspect_ratio) as usize;
        RenderSettings {
            aspect_ratio,
            image_width,
            image_height,
            sample_per_pixel: 100,
            max_depth: 50,
        }
    }
}

pub fn render(world: &Vec<Box<dyn Hittable>>, cam: &Camera, settings: &RenderSettings) -> Vec<u8> {
    let bar = &Box::new(ProgressBar::new(
        (settings.image_width * settings.image_height) as u64,
    ));
    bar.set_prefix("Render in progress");
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{prefix:.white} {bar:40.cyan/blue} {percent}% [{eta_precise}]"),
    );

    let result = (0..settings.image_height)
        .into_par_iter()
        .rev()
        .flat_map(|j| {
            (0..settings.image_width)
                .into_par_iter()
                .flat_map(move |i| {
                    let mut pixel_color = Color::default();
                    let mut rng = rand::thread_rng();
                    let dist = Uniform::from(0.0..=1.0);

                    for _ in 0..settings.sample_per_pixel {
                        let u =
                            (i as f64 + dist.sample(&mut rng)) / (settings.image_width - 1) as f64;
                        let v =
                            (j as f64 + dist.sample(&mut rng)) / (settings.image_height - 1) as f64;
                        let ray = cam.get_ray(u, v);
                        pixel_color += ray_color(ray, world, settings.max_depth);
                    }
                    bar.inc(1);
                    pixel_color /= settings.sample_per_pixel as f64;
                    pixel_color = color!(
                        pixel_color[0].sqrt(),
                        pixel_color[1].sqrt(),
                        pixel_color[2].sqrt()
                    );
                    (0..3)
                        .into_par_iter()
                        .map(move |k| (255.999 * pixel_color[k as usize]).min(255.0) as u8)
                })
        })
        .collect();
    bar.finish();

    result
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
