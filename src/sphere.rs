use crate::aabb::Aabb;
use crate::hit::Hit;
use crate::hit::Hittable;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};
use std::ops::RangeInclusive;
use std::sync::Arc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, range: RangeInclusive<f64>) -> Option<Hit> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = dot(oc, ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if !range.contains(&root) {
            root = (-half_b + sqrtd) / a;
            if !range.contains(&root) {
                return None;
            }
        }

        let outward_normal = (ray.at(root) - self.center) / self.radius;
        Some(Hit::new(
            ray.at(root),
            outward_normal,
            root,
            ray,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Some(Aabb::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        ))
    }
}
