use super::Material;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};
use crate::hit::Hit;

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit: &Hit, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = hit.normal + Vec3::random_unit();
        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }

        *scattered = Ray::new(hit.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}
