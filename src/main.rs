mod vec3;
mod ray;
mod hittable;
mod sphere;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::Hittable;
use crate::sphere::Sphere;

fn ray_color(ray: &Ray, world: &Vec<Box<dyn Hittable>>) -> Vec3 {
    match world.hit(ray, 0.0, f64::INFINITY) {
        Some(hit_record) => return (hit_record.normal + Vec3::new(1.0, 1.0, 1.0)) / 2.0,
        None => (),
    };

    let unit_direction = ray.direction.normalized();
    let a = (unit_direction.y + 1.0) / 2.0;

    // Linear interpolation
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
}

// ---------------------------------------------------------------------------

fn main() {
    let ideal_aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let image_height = (image_width as f64 / ideal_aspect_ratio) as i32;
    let image_height = i32::max(image_height, 1);

    let mut world: Vec<Box<dyn Hittable>> = Vec::new();
    world.push(Box::new(
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5))
    );
    world.push(Box::new(
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0))
    );

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * image_width as f64 / image_height as f64;
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel_00_location = viewport_upper_left + (pixel_delta_u + pixel_delta_v) / 2.0;

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in 0..image_height {
        eprintln!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_center = pixel_00_location + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray {
                origin: camera_center,
                direction: ray_direction,
            };

            let color = ray_color(&ray, &world);

            color.println();
        }
    }
    eprintln!("Done!");
}
