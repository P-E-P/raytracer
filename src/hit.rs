use std::sync::Arc;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::*;
use std::ops::RangeInclusive;

pub struct Hit {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl Hit {
    pub fn new(
        p: Point3,
        outward_normal: Vec3,
        t: f64,
        ray: Ray,
        material: Arc<dyn Material>,
    ) -> Self {
        let front_face = dot(ray.direction(), outward_normal) < 0.0;
        Hit {
            p,
            material,
            t,
            front_face,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, range: RangeInclusive<f64>) -> Option<Hit>;
}
