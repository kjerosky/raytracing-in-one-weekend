use crate::{ray::Ray, vec3::Vec3};

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t_min: f64, ray_t_max: f64) -> Option<HitRecord>;
}
