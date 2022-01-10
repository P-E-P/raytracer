use crate::Color;
use crate::Camera;
use crate::Hittable;

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
        let image_width = 200;
        let image_height = (image_width as f64 / aspect_ratio) as usize;
        RenderSettings {
            aspect_ratio,
            image_width,
            image_height,
            sample_per_pixel : 100,
            max_depth : 25,
        }
    }
}

pub fn render(world: &impl Hittable, cam: &Camera, settings: RenderSettings) -> Vec<Color> {
    vec![]
}

