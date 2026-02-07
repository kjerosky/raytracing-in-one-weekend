mod vec3;
mod ray;
mod hittable;
mod sphere;

use crate::vec3::Vec3;
use crate::ray::Ray;

fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray) -> f64 {
    let oc = *center - ray.origin;
    let a = ray.direction.length_squared();
    let h = Vec3::dot(&ray.direction, &oc);
    let c = oc.length_squared() - radius * radius;

    let discriminant = h * h - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (h - discriminant.sqrt()) / a
    }
}

// ---------------------------------------------------------------------------

fn ray_color(ray: &Ray) -> Vec3 {
    let t = hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let normal = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).normalized();
        return Vec3::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0) / 2.0;
    }

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

            let color = ray_color(&ray);

            color.println();
        }
    }
    eprintln!("Done!");
}
