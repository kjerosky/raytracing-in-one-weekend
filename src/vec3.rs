pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    // -----------------------------------------------------------------------

    pub fn println(&self) {
        let x = (255.999 * self.x) as u8;
        let y = (255.999 * self.y) as u8;
        let z = (255.999 * self.z) as u8;

        println!("{x} {y} {z}");
    }
}
