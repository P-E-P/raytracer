use crate::hit::Hit;
use crate::ray::Ray;
use crate::vec3::Color;

pub mod dielectric;
pub mod lambertian;
pub mod metal;

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<(Ray, Color)>;
}
