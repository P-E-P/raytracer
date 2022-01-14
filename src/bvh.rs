use crate::aabb::Aabb;
use crate::hit::{Hit, Hittable};
use crate::ray::Ray;
use crate::utils::random;
use std::cmp::Ordering;
use std::sync::Arc;

pub struct Bvh {
    right: Arc<dyn Hittable>,
    left: Arc<dyn Hittable>,
    aabb: Aabb,
}

impl Bvh {
    fn new(src_objects: &Vec<Arc<dyn Hittable>>, time0: f64, time1: f64) -> Self {
        Self::bounded(src_objects, 0, src_objects.len(), time0, time1)
    }

    fn bounded(
        src_objects: &Vec<Arc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let mut objects = src_objects.clone();
        let axis = random(0..=2);

        let comparator = if axis == 0 {
            box_x_compare
        } else if axis == 1 {
            box_y_compare
        } else {
            box_z_compare
        };

        let object_span = end - start;

        let (left, right) = if object_span == 1 {
            (objects[start].clone(), objects[start].clone())
        } else if object_span == 2 {
            if comparator(&objects[start], &objects[start + 1]).is_lt() {
                (objects[start].clone(), objects[start + 1].clone())
            } else {
                (objects[start + 1].clone(), objects[start].clone())
            }
        } else {
            objects[start..end].sort_unstable_by(comparator);
            let mid = start + object_span / 2;
            let left: Arc<dyn Hittable> =
                Arc::new(Bvh::bounded(&objects, start, mid, time0, time1));
            let right: Arc<dyn Hittable> = Arc::new(Bvh::bounded(&objects, mid, end, time0, time1));
            (left, right)
        };

        let (box_left, box_right) = match (
            left.bounding_box(time0, time1),
            right.bounding_box(time0, time1),
        ) {
            (Some(box_left), Some(box_right)) => (box_left, box_right),
            _ => panic!("No bounding box in bvh_node constructor"),
        };

        Bvh {
            left,
            right,
            aabb: Aabb::surrounding(&box_left, &box_right),
        }
    }
}

fn box_compare(
    a: &Arc<dyn Hittable>,
    b: &Arc<dyn Hittable>,
    axis: usize,
) -> Ordering {
    let (box_a, box_b) = match (a.bounding_box(0.0, 0.0), b.bounding_box(0.0, 0.0)) {
        (Some(box_a), Some(box_b)) => (box_a, box_b),
        _ => panic!("No bounding box in bvh_node constructor"),
    };
    box_a.min().e[axis]
        .partial_cmp(&box_b.min().e[axis])
        .unwrap()
}

fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2)
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
