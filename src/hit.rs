use crate::vec3::*;
use crate::ray::Ray;

pub struct Hit {
    p: Point3,
    normal: Vec3,
    t: f64,
}

trait Hittable {
    fn hit(ray: Ray, t_min: f64, t_max: f64, rec: Hit) -> bool;
}
