use crate::vec3::*;

#[derive(Copy, Clone)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
    tm: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
            tm: 0.0,
        }
    }

    pub fn timed(origin: Point3, direction: Vec3, time: f64) -> Self {
        Self {
            origin,
            direction,
            tm: time,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn time(&self) -> f64 {
        self.tm
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
