use super::Material;
use crate::hit::Hit;
use crate::ray::Ray;
use crate::vec3::{dot, reflect, unit_vector, Color, Vec3};

pub struct Metal {
    pub albedo: Color,
    pub fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzziness: f64) -> Self {
        Metal {
            albedo,
            fuzziness: if fuzziness < 1.0 { fuzziness } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
        let reflected = reflect(unit_vector(r_in.direction()), hit.normal);
        let scattered = Ray::new(
            hit.p,
            reflected + self.fuzziness * Vec3::random_in_unit_sphere(),
        )
        .timed(r_in.time());
        if dot(scattered.direction(), hit.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
