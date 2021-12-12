use crate::hit::Hit;
use crate::hit::Hittable;
use crate::ray::Ray;
use crate::vec3::{dot, Point3};
use std::ops::RangeBounds;

struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    fn new(center: Point3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, range: impl RangeBounds<f64>) -> Option<Hit> {
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

        let mut result = Hit::new(
            ray.at(root),
            (ray.at(root) - self.center) / self.radius,
            root,
        );
        let outward_normal = (ray.at(root) - self.center) / self.radius;
        result.set_face_normal(ray, outward_normal);
        Some(result)
    }
}
