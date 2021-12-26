use crate::aabb::Aabb;
use crate::hit::{Hit, Hittable};
use crate::ray::Ray;
use std::sync::Arc;

pub struct Bvh {
    right: Arc<dyn Hittable>,
    left: Arc<dyn Hittable>,
    aabb: Aabb,
}

impl Bvh {
    /*fn new(objects: Vec<Arc<dyn Hittable>>, start: usize, end: usize, time0: f64, time1: f64) -> Self {

    }*/
}

impl Hittable for Bvh {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        if self.aabb.hit(ray, t_min, t_max) {
            let hit_left = self.left.hit(ray, t_min, t_max);
            let hit_right = match hit_left {
                Some(ref hit) => self.right.hit(ray, t_min, hit.t),
                None => self.right.hit(ray, t_min, t_max),
            };
            hit_right.or(hit_left)
        } else {
            None
        }
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Some(self.aabb)
    }
}
