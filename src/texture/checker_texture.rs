use super::{solid_color::SolidColor, Texture};
use crate::vec3::Color;

pub struct CheckerTexture {
    inv_scale: f64,
    even: Box<dyn Texture>,
    odd: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Box<dyn Texture>, odd: Box<dyn Texture>) -> Self {
        CheckerTexture {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn from_color(scale: f64, c1: Color, c2: Color) -> Self {
        CheckerTexture {
            inv_scale: 1.0 / scale,
            even: Box::new(SolidColor::from(c1)),
            odd: Box::new(SolidColor::from(c2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: crate::vec3::Point3) -> crate::vec3::Color {
        let x = f64::floor(self.inv_scale * p.x()) as i32;
        let y = f64::floor(self.inv_scale * p.y()) as i32;
        let z = f64::floor(self.inv_scale * p.z()) as i32;

        if (x + y + z) % 2 == 0 {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
