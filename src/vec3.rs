use rand::distributions::{Distribution, Uniform};
use std::fmt;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, RangeInclusive, Sub,
};

pub type Point3 = Vec3;
pub type Color = Vec3;

#[macro_export]
macro_rules! color {
    ($x:expr, $y:expr,$z:expr) => {
        crate::vec3::Color::new($x, $y, $z)
    };
}

#[macro_export]
macro_rules! point {
    ($x:expr, $y:expr,$z:expr) => {
        crate::vec3::Point3::new($x, $y, $z)
    };
}

#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr,$z:expr) => {
        crate::vec3::Vec3::new($x, $y, $z)
    };
}

#[derive(Copy, Clone, Default)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { e: [x, y, z] }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let dist = Uniform::from(0.0..=1.0);
        Vec3 {
            e: [
                dist.sample(&mut rng),
                dist.sample(&mut rng),
                dist.sample(&mut rng),
            ],
        }
    }

    pub fn delimited(range: RangeInclusive<f64>) -> Self {
        let mut rng = rand::thread_rng();
        let dist = Uniform::from(range);
        Vec3 {
            e: [
                dist.sample(&mut rng),
                dist.sample(&mut rng),
                dist.sample(&mut rng),
            ],
        }
    }

    pub fn random_in_unit_disk() -> Self {
        let mut rng = rand::thread_rng();
        let dist = Uniform::from(-1.0..=1.0);
        loop {
            let p = Vec3::new(dist.sample(&mut rng), dist.sample(&mut rng), 0.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    /// A (bad) diffuse renderer
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::delimited(-1.0..=1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    /// Intuitive diffuse renderer
    pub fn random_in_hemisphere(normal: Vec3) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if dot(in_unit_sphere, normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    /// Accurate diffuse renderer.
    pub fn random_unit() -> Self {
        unit_vector(Self::random_in_unit_sphere())
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn len(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn near_zero(&self) -> bool {
        const S: f64 = 1e-8;
        (self.e[0].abs() < S) && (self.e[1].abs() < S) && (self.e[2].abs() < S)
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.e[0] = self.e[0] + other.e[0];
        self.e[1] = self.e[1] + other.e[1];
        self.e[2] = self.e[2] + other.e[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.e[0] *= 1.0 / rhs;
        self.e[1] *= 1.0 / rhs;
        self.e[2] *= 1.0 / rhs;
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        Self {
            e: [other * self.e[0], other * self.e[1], other * self.e[2]],
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3 {
            e: [self * other.e[0], self * other.e[1], self * other.e[2]],
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Self::Output {
        (1.0 / other) * self
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.len()
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.y() * v.z() - u.z() * v.y(),
        u.z() * v.x() - u.x() * v.z(),
        u.x() * v.y() - u.y() * v.x(),
    )
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min(dot(-uv, n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * n;
    r_out_perp + r_out_parallel
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}
