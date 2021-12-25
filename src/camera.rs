use crate::ray::Ray;
use crate::utils::random;
use crate::vec3::{cross, unit_vector, Point3, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        Self::timed(
            look_from,
            look_at,
            vup,
            vfov,
            aspect_ratio,
            aperture,
            focus_dist,
            0.0,
            0.0,
        )
    }

    pub fn timed(
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time0: f64,
        time1: f64,
    ) -> Self {
        let h = (vfov.to_radians() / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(look_from - look_at);
        let u = unit_vector(cross(vup, w));
        let v = cross(w, u);
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;

        Camera {
            u,
            v,
            w,
            origin: look_from,
            horizontal,
            vertical,
            lens_radius: aperture / 2.0,
            lower_left_corner: look_from - horizontal / 2.0 - vertical / 2.0 - focus_dist * w,
            time0,
            time1,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::timed(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            random(self.time0..=self.time1),
        )
    }
}
