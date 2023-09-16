use std::cmp::Ordering;

use crate::aabb::Interval;
use crate::ray::Ray;
use crate::utils;
use crate::Aabb;
use crate::Hittable;
use crate::HittableList;
use std::sync::Arc;

pub struct Bvh {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis_index: usize) -> Ordering {
    a.bounding_box()
        .axis(axis_index)
        .minimum
        .total_cmp(&b.bounding_box().axis(axis_index).minimum)
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

impl Bvh {
    pub fn new(list: HittableList) -> Self {
        Self::from_slice(&list.objects)
    }

    pub fn from_slice(src_objects: &[Arc<dyn Hittable>]) -> Self {
        let mut objects = src_objects.to_vec();
        let axis = utils::random(0..=2);
        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            2 => box_z_compare,
            _ => unreachable!(),
        };

        let (left, right) = if objects.len() == 1 {
            (objects[0].clone(), objects[0].clone())
        } else if objects.len() == 2 {
            if comparator(&objects[0], &objects[1]) == Ordering::Less {
                (objects[0].clone(), objects[1].clone())
            } else {
                (objects[1].clone(), objects[0].clone())
            }
        } else {
            objects.sort_unstable_by(|a, b| comparator(a, b));
            let mid = objects.len() / 2;
            (
                Arc::new(Bvh::from_slice(&objects[0..mid])) as Arc<dyn Hittable>,
                Arc::new(Bvh::from_slice(&objects[mid..objects.len()])) as Arc<dyn Hittable>,
            )
        };
        let bbox = Aabb::surrounding(&left.bounding_box(), &right.bounding_box());

        Bvh { left, right, bbox }
    }
}

impl Hittable for Bvh {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<crate::hit::Hit> {
        if !self.bbox.hit(ray, ray_t) {
            return None;
        }

        let hit_left = self.left.hit(ray, ray_t);
        let hit_right = self.right.hit(
            ray,
            Interval::new(
                ray_t.minimum,
                if let Some(value) = &hit_left {
                    value.t
                } else {
                    ray_t.maximum
                },
            ),
        );

        hit_right.or(hit_left)
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
