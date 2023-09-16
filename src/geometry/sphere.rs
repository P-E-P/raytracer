use crate::aabb::Aabb;
use crate::aabb::Interval;
use crate::hit::Hit;
use crate::hit::Hittable;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3;
use crate::vec3::{dot, Point3, Vec3};

pub struct Sphere<M: Material> {
    center: Point3,
    radius: f64,
    material: M,
    is_moving: bool,
    center_vec: Vec3,
    bbox: Aabb,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Point3, radius: f64, material: M) -> Self {
        let rvec = vec3!(radius, radius, radius);
        Sphere {
            center,
            radius,
            material,
            is_moving: false,
            center_vec: Vec3::default(),
            bbox: Aabb::from_extremum(center - rvec, center + rvec),
        }
    }

    pub fn moving(center_1: Point3, center_2: Point3, radius: f64, material: M) -> Self {
        let rvec = vec3!(radius, radius, radius);
        let box1 = Aabb::from_extremum(center_1 - rvec, center_1 + rvec);
        let box2 = Aabb::from_extremum(center_2 - rvec, center_2 + rvec);
        Sphere {
            center: center_1,
            radius,
            material,
            is_moving: true,
            center_vec: center_2 - center_1,
            bbox: Aabb::surrounding(&box1, &box2),
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        self.center + time * self.center_vec
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<Hit> {
        let center = if self.is_moving {
            self.center(ray.time())
        } else {
            self.center
        };

        let oc = ray.origin() - center;
        let a = ray.direction().length_squared();
        let half_b = dot(oc, ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.contains(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.contains(root) {
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
}
