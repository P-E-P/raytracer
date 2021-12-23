use super::Material;
use crate::hit::Hit;
use crate::ray::Ray;
use crate::vec3::{refract, unit_vector, Color};

pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Dielectric {
            refraction_index,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit: &Hit, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = color!(1.0, 1.0, 1.0);
        let refraction_ratio = if hit.front_face { 1.0/ self.refraction_index } else { self.refraction_index };
        let unit_direction = unit_vector(r_in.direction());
        let refracted = refract(unit_direction, hit.normal, refraction_ratio);

        *scattered = Ray::new(hit.p, refracted);
        true
    }
}
