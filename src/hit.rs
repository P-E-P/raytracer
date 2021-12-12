use crate::ray::Ray;
use crate::vec3::*;

pub struct Hit {
    p: Point3,
    normal: Vec3,
    t: f64,
}

impl Hit {
    pub fn new(p: Point3, normal: Vec3, t: f64) -> Self {
        Hit { p, normal, t }
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}
