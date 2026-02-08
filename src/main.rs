mod vec3;
mod ray;
mod hittable;
mod sphere;
mod interval;
mod camera;

use crate::camera::Camera;
use crate::vec3::Vec3;
use crate::hittable::Hittable;
use crate::sphere::Sphere;

fn main() {
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();
    world.push(Box::new(
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)
    ));
    world.push(Box::new(
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)
    ));

    let mut camera = Camera::new(16.0 / 9.0, 400, 100);
    camera.render(&world);
}
