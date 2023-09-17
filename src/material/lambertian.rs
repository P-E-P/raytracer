use super::Material;
use crate::hit::Hit;
use crate::ray::Ray;
use crate::texture::solid_color::SolidColor;
use crate::texture::Texture;
use crate::vec3::{Color, Vec3};
use std::sync::Arc;

pub struct Lambertian {
    pub albedo: Box<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian {
            albedo: Box::new(SolidColor::from(albedo)),
        }
    }

    pub fn textured(texture: Box<dyn Texture>) -> Self {
        Lambertian { albedo: texture }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit.normal + Vec3::random_unit();
        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }

        let scattered = Ray::new(hit.p, scatter_direction).timed(r_in.time());
        Some((scattered, self.albedo.value(hit.u, hit.v, hit.p)))
    }
}
