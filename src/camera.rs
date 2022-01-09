use crate::ray::Ray;
use crate::utils::random;
use crate::vec3::{cross, unit_vector, Point3, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

pub struct Builder {
    origin: Point3,
    look_at: Point3,
    vup: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
    aspect_ratio: f64,
    focus_dist: f64,
    vfov: f64,
}

impl Builder {
    /// Create a new camera.
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        Builder {
            origin: look_from,
            look_at,
            vup: vec3!(0.0, 1.0, 0.0),
            lens_radius: aperture / 2.0,
            time0: 0.0,
            time1: 0.0,
            aspect_ratio,
            focus_dist,
            vfov,
        }
    }

    /// Tilt a camera by changing the direction of it's upward component.
    pub fn tilted(&mut self, vup: Vec3) -> &mut Builder {
        self.vup = vup;
        self
    }

    pub fn timed(&mut self, start: f64, end: f64) -> &mut Builder {
        self.time0 = start;
        self.time1 = end;
        self
    }

    pub fn build(&self) -> Camera {
        let h = (self.vfov.to_radians() / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = self.aspect_ratio * viewport_height;

        let w = unit_vector(self.origin - self.look_at);
        let u = unit_vector(cross(self.vup, w));
        let v = cross(w, u);
        let horizontal = self.focus_dist * viewport_width * u;
        let vertical = self.focus_dist * viewport_height * v;
        let lower_left_corner =
            self.origin - horizontal / 2.0 - vertical / 2.0 - self.focus_dist * w;

        Camera {
            origin: self.origin,
            lower_left_corner,
            u,
            v,
            w,
            horizontal,
            vertical,
            lens_radius: self.lens_radius,
            time0: self.time0,
            time1: self.time1,
        }
    }
}

impl Camera {
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
        .timed(random(self.time0..=self.time1))
    }
}
