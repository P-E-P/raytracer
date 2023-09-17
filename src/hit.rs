use crate::aabb::{Aabb, Interval};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::*;

pub struct Hit<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl<'a> Hit<'a> {
    pub fn new(
        p: Point3,
        outward_normal: Vec3,
        t: f64,
        ray: Ray,
        material: &'a dyn Material,
    ) -> Self {
        let front_face = dot(ray.direction(), outward_normal) < 0.0;
        Hit {
            p,
            material,
            t,
            u: 0.0,
            v: 0.0,
            front_face,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
        }
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<Hit>;
    fn bounding_box(&self) -> Aabb;
}
