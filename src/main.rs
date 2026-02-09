mod vec3;
mod ray;
mod hittable;
mod sphere;
mod interval;
mod camera;
mod material;

use std::rc::Rc;

use crate::camera::Camera;
use crate::material::{Lambertian, Metal};
use crate::vec3::Vec3;
use crate::hittable::Hittable;
use crate::sphere::Sphere;

fn main() {
    let material_ground = Rc::new(Lambertian { albedo: Vec3::new(0.8, 0.8, 0.0) });
    let material_center = Rc::new(Lambertian { albedo: Vec3::new(0.1, 0.2, 0.5) });
    let material_left = Rc::new(Metal { albedo: Vec3::new(0.8, 0.8, 0.8) });
    let material_right = Rc::new(Metal { albedo: Vec3::new(0.8, 0.6, 0.2) });

    let mut world: Vec<Box<dyn Hittable>> = Vec::new();
    world.push(Box::new(
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground)
    ));
    world.push(Box::new(
        Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, material_center)
    ));
    world.push(Box::new(
        Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left)
    ));
    world.push(Box::new(
        Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right)
    ));

    let mut camera = Camera::new(16.0 / 9.0, 400, 100, 50);
    camera.render(&world);
}
