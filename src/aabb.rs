use crate::ray::Ray;
use crate::vec3::Point3;

#[derive(Copy, Clone, Default)]
pub struct Interval {
    pub minimum: f64,
    pub maximum: f64,
}

impl Interval {
    pub fn new(minimum: f64, maximum: f64) -> Self {
        Interval { minimum, maximum }
    }

    pub fn enclosing(first: &Interval, second: &Interval) -> Self {
        Interval {
            minimum: f64::min(first.minimum, second.minimum),
            maximum: f64::max(first.maximum, second.maximum),
        }
    }

    pub fn size(&self) -> f64 {
        return self.maximum - self.minimum;
    }

    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Interval {
            minimum: self.minimum - padding,
            maximum: self.maximum + padding,
        }
    }

    pub fn contains(&self, value: f64) -> bool {
        (self.minimum..=self.maximum).contains(&value)
    }
}

#[derive(Copy, Clone, Default)]
pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Aabb { x, y, z }
    }

    pub fn from_extremum(a: Point3, b: Point3) -> Self {
        Aabb {
            x: Interval {
                minimum: f64::min(a.x(), b.x()),
                maximum: f64::max(a.x(), b.x()),
            },
            y: Interval {
                minimum: f64::min(a.y(), b.y()),
                maximum: f64::max(a.y(), b.y()),
            },
            z: Interval {
                minimum: f64::min(a.z(), b.z()),
                maximum: f64::max(a.z(), b.z()),
            },
        }
    }

    pub fn surrounding(first: &Aabb, second: &Aabb) -> Self {
        Aabb {
            x: Interval::enclosing(&first.x, &second.x),
            y: Interval::enclosing(&first.y, &second.y),
            z: Interval::enclosing(&first.z, &second.z),
        }
    }

    pub fn axis(&self, n: usize) -> &Interval {
        match n {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => unreachable!(),
        }
    }

    pub fn hit(&self, ray: Ray, mut ray_t: Interval) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.direction()[a];
            let origin = ray.origin()[a];
            let mut t0 = (self.axis(a).minimum - origin) * inv_d;
            let mut t1 = (self.axis(a).maximum - origin) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            if t0 > ray_t.minimum {
                ray_t.minimum = t0;
            }
            if t1 < ray_t.maximum {
                ray_t.maximum = t1;
            }

            if ray_t.maximum <= ray_t.minimum {
                return false;
            }
        }
        true
    }
}
