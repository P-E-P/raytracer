use super::Texture;
use crate::vec3::{Color, Point3};

pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn from_rgb(red: f64, green: f64, blue: f64) -> Self {
        SolidColor::from(Color::new(red, green, blue))
    }
}

impl From<Color> for SolidColor {
    fn from(color_value: Color) -> Self {
        SolidColor { color_value }
    }
}

impl Texture for SolidColor {
    fn value(&self, _: f64, _: f64, _: Point3) -> Color {
        self.color_value
    }
}
