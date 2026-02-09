use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub};

use rand::{Rng, rngs::ThreadRng};

use crate::interval::Interval;

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// ---------------------------------------------------------------------------

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

// ---------------------------------------------------------------------------

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

// ---------------------------------------------------------------------------

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

// ---------------------------------------------------------------------------

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

// ---------------------------------------------------------------------------

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

// ---------------------------------------------------------------------------

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

// ---------------------------------------------------------------------------

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// ---------------------------------------------------------------------------

impl Vec3 {
    pub fn new_zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    // -----------------------------------------------------------------------

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    // -----------------------------------------------------------------------

    pub fn dot(a: &Self, b: &Self) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    // -----------------------------------------------------------------------

    pub fn reflect(v: &Self, n: &Self) -> Self {
        *v - *n * 2.0 * Self::dot(v, n)
    }

    // -----------------------------------------------------------------------

    fn random(rng: &mut ThreadRng, min: f64, max: f64) -> Self {
        Self {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
            z: rng.gen_range(min..max),
        }
    }

    // -----------------------------------------------------------------------

    pub fn random_unit_vector(rng: &mut ThreadRng) -> Self {
        loop {
            let p = Self::random(rng, -1.0, 1.0);
            let length_squared = p.length_squared();
            if 1e-160 < length_squared && length_squared <= 1.0 {
                return p / length_squared.sqrt();
            }
        }
    }

    // -----------------------------------------------------------------------

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    // -----------------------------------------------------------------------

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    // -----------------------------------------------------------------------

    pub fn normalized(&self) -> Self {
        *self / self.length()
    }

    // -----------------------------------------------------------------------

    fn linear_to_gamma(&self, linear_component: f64) -> f64 {
        if linear_component > 0.0 {
            linear_component.sqrt()
        } else {
            0.0
        }
    }

    // -----------------------------------------------------------------------

    pub fn is_near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    // -----------------------------------------------------------------------

    pub fn println(&self) {
        let r = self.linear_to_gamma(self.x);
        let g = self.linear_to_gamma(self.y);
        let b = self.linear_to_gamma(self.z);

        let intensity_range = 0.000..=0.999;
        let x = (256.0 * intensity_range.clamp(&r)) as u8;
        let y = (256.0 * intensity_range.clamp(&g)) as u8;
        let z = (256.0 * intensity_range.clamp(&b)) as u8;

        println!("{x} {y} {z}");
    }
}
