use crate::ray::Ray;
use crate::vec3::*;
use std::ops::RangeBounds;

pub struct Hit {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl Hit {
    pub fn new(p: Point3, normal: Vec3, t: f64) -> Self {
        Hit {
            p,
            normal,
            t,
            front_face: true,
        }
    }

    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        self.front_face = dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, range: impl RangeBounds<f64>) -> Option<Hit>;
}
