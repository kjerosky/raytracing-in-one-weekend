use std::{ops::RangeInclusive, rc::Rc};

use crate::{material::Material, ray::Ray, vec3::Vec3};

pub struct HitRecord {
    pub point: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f64,

    pub normal: Vec3,
    pub front_face: bool,
}

// ---------------------------------------------------------------------------

impl HitRecord {
    pub fn new(point: Vec3, t: f64, ray: &Ray, outward_normal: Vec3, material: Rc<dyn Material>) -> Self {
        // NOTE: `outward_normal` is assumed to have unit_length.
        let front_face = Vec3::dot(&ray.direction, &outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };

        Self {
            point,
            material,
            t,
            normal,
            front_face,
        }
    }
}

// ---------------------------------------------------------------------------

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t_range: RangeInclusive<f64>) -> Option<HitRecord>;
}

// ---------------------------------------------------------------------------

impl Hittable for Vec<Box<dyn Hittable>> {
    fn hit(&self, ray: &Ray, ray_t_range: RangeInclusive<f64>) -> Option<HitRecord> {
        let mut result = None;
        let mut closest_t_so_far = *ray_t_range.end();

        for object in self {
            match object.hit(ray, *ray_t_range.start()..=closest_t_so_far) {
                Some(hit_record) => {
                    closest_t_so_far = hit_record.t;
                    result = Some(hit_record);
                },
                None => (),
            };
        }

        return result;
    }
}
