use crate::aabb::Aabb;
use crate::hit::Hit;
use crate::hit::Hittable;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};
use std::sync::Arc;

pub struct Sphere<M: Material> {
    center: Point3,
    radius: f64,
    material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Point3, radius: f64, material: M) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit> {
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
        if !(t_min..=t_max).contains(&root) {
            root = (-half_b + sqrtd) / a;
            if !(t_min..=t_max).contains(&root) {
                return None;
            }
        }

        let outward_normal = (ray.at(root) - self.center) / self.radius;
        Some(Hit::new(
            ray.at(root),
            outward_normal,
            root,
            ray,
            &self.material,
        ))
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Some(Aabb::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        ))
    }
}
