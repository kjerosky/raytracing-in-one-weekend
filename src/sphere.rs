use crate::{hittable::{HitRecord, Hittable}, vec3::Vec3};

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, ray_t_min: f64, ray_t_max: f64) -> Option<HitRecord> {
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
        if root <= ray_t_min || root >= ray_t_max {
            root = (h + sqrt_discriminant) / a;
            if root <= ray_t_min || root >= ray_t_max {
                return None;
            }
        }

        let t = root;
        let point = ray.at(t);
        let normal = (point - self.center) / self.radius;
        let hit_record = HitRecord {
            point,
            normal,
            t,
        };
        Some(hit_record)
    }
}
