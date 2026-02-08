use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub};

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

    pub fn dot(a: &Vec3, b: &Vec3) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
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

    pub fn println(&self) {
        let intensity_range = 0.000..=0.999;
        let x = (256.0 * intensity_range.clamp(&self.x)) as u8;
        let y = (256.0 * intensity_range.clamp(&self.y)) as u8;
        let z = (256.0 * intensity_range.clamp(&self.z)) as u8;

        println!("{x} {y} {z}");
    }
}
