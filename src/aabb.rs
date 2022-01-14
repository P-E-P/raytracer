use crate::ray::Ray;
use crate::utils::{max, min};
use crate::vec3::Point3;

#[derive(Copy, Clone)]
pub struct Aabb {
    minimum: Point3,
    maximum: Point3,
}

impl Aabb {
    pub fn new(a: Point3, b: Point3) -> Self {
        Aabb {
            minimum: a,
            maximum: b,
        }
    }

    pub fn surrounding(box0: &Aabb, box1: &Aabb) -> Self {
        let small = Point3::new(
            min(box0.min().x(), box1.min().x()),
            min(box0.min().y(), box1.min().y()),
            min(box0.min().z(), box1.min().z()),
        );

        let big = Point3::new(
            max(box0.max().x(), box1.max().x()),
            max(box0.max().y(), box1.max().y()),
            max(box0.max().z(), box1.max().z()),
        );

        Aabb::new(small, big)
    }

    pub fn min(&self) -> Point3 {
        self.minimum
    }

    pub fn max(&self) -> Point3 {
        self.maximum
    }

    pub fn hit(&self, ray: Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.direction()[a];
            let t0 = (self.min()[a] - ray.origin()[a]) * inv_d;
            let t1 = (self.max()[a] - ray.origin()[a]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t_min, &mut t_max)
            }
            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}
