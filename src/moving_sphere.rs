use crate::hit::Hit;
use crate::hit::Hittable;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Point3};
use std::ops::RangeInclusive;
use std::sync::Arc;

pub struct MovingSphere {
    center0: Point3,
    center1: Point3,
    time0: f64,
    time1: f64,
    radius: f64,
    material: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(center0: Point3, center1: Point3, time0: f64, time1: f64, radius: f64, material: Arc<dyn Material>) -> Self {
        MovingSphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    fn center(&self, time: f64) -> Point3 {
        self.center0 + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }

}

impl Hittable for MovingSphere {
    fn hit(&self, ray: Ray, range: RangeInclusive<f64>) -> Option<Hit> {
        let oc = ray.origin() - self.center(ray.time());
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

        let outward_normal = (ray.at(root) - self.center(ray.time())) / self.radius;
        Some(Hit::new(
            ray.at(root),
            outward_normal,
            root,
            ray,
            self.material.clone(),
        ))
    }
}
