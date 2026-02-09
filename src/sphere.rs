use std::{ops::RangeInclusive, rc::Rc};

use crate::{hittable::{HitRecord, Hittable}, interval::Interval, material::Material, vec3::Vec3};

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Rc<dyn Material>,
}

// ---------------------------------------------------------------------------

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
            material,
        }
    }
}

// ---------------------------------------------------------------------------

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, ray_t_range: RangeInclusive<f64>) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.direction.length_squared();
        let h = Vec3::dot(&ray.direction, &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrt_discriminant) / a;
        if !ray_t_range.surrounds(&root) {
            root = (h + sqrt_discriminant) / a;
            if !ray_t_range.surrounds(&root) {
                return None;
            }
        }

        let t = root;
        let point = ray.at(t);
        let outward_normal = (point - self.center) / self.radius;
        let hit_record = HitRecord::new(
            point,
            t,
            ray,
            outward_normal,
            Rc::clone(&self.material)
        );
        Some(hit_record)
    }
}
