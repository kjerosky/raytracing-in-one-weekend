use std::ops::{Add, Div, Mul, Sub};

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

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
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
        let x = (255.999 * self.x) as u8;
        let y = (255.999 * self.y) as u8;
        let z = (255.999 * self.z) as u8;

        println!("{x} {y} {z}");
    }
}
