use crate::ray::Ray;
use super::Material;
use crate::vec3::{Color, reflect, dot, unit_vector};
use crate::hit::Hit;

pub struct Metal {
    pub albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &Hit, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = reflect(unit_vector(r_in.direction()), hit.normal);
        *scattered = Ray::new(hit.p, reflected);
        *attenuation = self.albedo;
        dot(scattered.direction(), hit.normal) > 0.0
    }
}
