use super::Material;
use crate::hit::Hit;
use crate::ray::Ray;
use crate::utils::{min, random};
use crate::vec3::{dot, reflect, refract, unit_vector, Color};

pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Dielectric { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit: &Hit, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = color!(1.0, 1.0, 1.0);
        let refraction_ratio = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = min(dot(-unit_direction, hit.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).abs().sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > random(0.0..=1.0) {
                reflect(unit_direction, hit.normal)
            } else {
                refract(unit_direction, hit.normal, refraction_ratio)
            };

        *scattered = Ray::new(hit.p, direction).timed(r_in.time());
        true
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
