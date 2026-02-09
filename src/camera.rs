use rand::{Rng, rngs::ThreadRng};

use crate::{hittable::Hittable, ray::Ray, vec3::Vec3};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_ray_bounces_allowed: i32,

    image_height: i32,
    pixel_samples_scale: f64,
    rng: ThreadRng,
    center: Vec3,
    pixel_00_location: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

// ---------------------------------------------------------------------------

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32, samples_per_pixel: i32, max_ray_bounces_allowed: i32) -> Self {
        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_ray_bounces_allowed,

            image_height: 0,
            pixel_samples_scale: 0.0,
            rng: rand::thread_rng(),
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
                let mut pixel_color = Vec3::new_zero();
                for _sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += self.ray_color(&ray, self.max_ray_bounces_allowed, world);
                }

                pixel_color *= self.pixel_samples_scale;
                pixel_color.println();
            }
        }
        eprintln!("Done!");
    }

    // -----------------------------------------------------------------------

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = i32::max(1, self.image_height);

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

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

    fn ray_color(&mut self, ray: &Ray, ray_bounces_allowed: i32, world: &Vec<Box<dyn Hittable>>) -> Vec3 {
        if ray_bounces_allowed <= 0 {
            return Vec3::new_zero();
        }

        match world.hit(ray, 0.001..=f64::INFINITY) {
            Some(hit_record) => {
                match hit_record.material.scatter(ray, &hit_record, &mut self.rng) {
                    Some(material_hit) => {
                        let attenuation = material_hit.attenuation;
                        let color = self.ray_color(&material_hit.scattered_ray, ray_bounces_allowed - 1, world);
                        return Vec3::new(
                            attenuation.x * color.x,
                            attenuation.y * color.y,
                            attenuation.z * color.z,
                        );
                    },
                    None => return Vec3::new_zero(),
                };
            }
            None => (),
        };

        let unit_direction = ray.direction.normalized();
        let a = (unit_direction.y + 1.0) / 2.0;

        // Linear interpolation
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
    }

    // -----------------------------------------------------------------------

    fn get_ray(&mut self, pixel_x: i32, pixel_y: i32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel_00_location
            + self.pixel_delta_u * (pixel_x as f64 + offset.x)
            + self.pixel_delta_v * (pixel_y as f64 + offset.y);

        let origin = self.center;
        let direction = pixel_sample - origin;

        Ray {
            origin,
            direction,
        }
    }

    // -----------------------------------------------------------------------

    fn sample_square(&mut self) -> Vec3 {
        // Return the vector to a random point in the [-0.5, -0.5]-[+0.5, +0.5] unit square.
        Vec3::new(
            self.rng.gen_range(-0.5..=0.5),
            self.rng.gen_range(-0.5..=0.5),
            0.0
        )
    }
}
