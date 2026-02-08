use crate::{hittable::Hittable, ray::Ray, vec3::Vec3};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,

    image_height: i32,
    center: Vec3,
    pixel_00_location: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

// ---------------------------------------------------------------------------

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32) -> Self {
        Self {
            aspect_ratio,
            image_width,

            image_height: 0,
            center: Vec3::new_zero(),
            pixel_00_location: Vec3::new_zero(),
            pixel_delta_u: Vec3::new_zero(),
            pixel_delta_v: Vec3::new_zero(),
        }
    }

    // -----------------------------------------------------------------------

    pub fn render(&mut self, world: &Vec<Box<dyn Hittable>>) {
        self.initialize();

        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for j in 0..self.image_height {
            eprintln!("Scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let pixel_center = self.pixel_00_location + (self.pixel_delta_u * i as f64) + (self.pixel_delta_v * j as f64);
                let ray_direction = pixel_center - self.center;
                let ray = Ray {
                    origin: self.center,
                    direction: ray_direction,
                };

                let color = self.ray_color(&ray, &world);

                color.println();
            }
        }
        eprintln!("Done!");
    }

    // -----------------------------------------------------------------------

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = i32::max(1, self.image_height);

        self.center = Vec3::new_zero();

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left = self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel_00_location = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) / 2.0;
    }

    // -----------------------------------------------------------------------

    fn ray_color(&self, ray: &Ray, world: &Vec<Box<dyn Hittable>>) -> Vec3 {
        match world.hit(ray, 0.0..=f64::INFINITY) {
            Some(hit_record) => return (hit_record.normal + Vec3::new(1.0, 1.0, 1.0)) / 2.0,
            None => (),
        };

        let unit_direction = ray.direction.normalized();
        let a = (unit_direction.y + 1.0) / 2.0;

        // Linear interpolation
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
    }
}
