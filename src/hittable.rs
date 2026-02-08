use crate::{ray::Ray, vec3::Vec3};

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

// ---------------------------------------------------------------------------

impl HitRecord {
    pub fn new(point: Vec3, t: f64, ray: &Ray, outward_normal: Vec3) -> Self {
        // NOTE: `outward_normal` is assumed to have unit_length.
        let front_face = Vec3::dot(&ray.direction, &outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };

        Self {
            point,
            normal,
            t,
            front_face,
        }
    }
}

// ---------------------------------------------------------------------------

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t_min: f64, ray_t_max: f64) -> Option<HitRecord>;
}
