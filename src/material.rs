use rand::rngs::ThreadRng;

use crate::{hittable::HitRecord, ray::Ray, vec3::Vec3};

pub struct MaterialHit {
    pub attenuation: Vec3,
    pub scattered_ray: Ray,
}

// ---------------------------------------------------------------------------

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, rng: &mut ThreadRng) -> Option<MaterialHit>;
}

// ---------------------------------------------------------------------------

pub struct Lambertian {
    pub albedo: Vec3,
}

// ---------------------------------------------------------------------------

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord, rng: &mut ThreadRng) -> Option<MaterialHit> {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector(rng);

        // Catch degenerate scatter direction
        if scatter_direction.is_near_zero() {
            scatter_direction = hit_record.normal;
        }

        let attenuation = self.albedo;
        let scattered_ray = Ray {
            origin: hit_record.point,
            direction: scatter_direction,
        };

        Some(MaterialHit {
            attenuation,
            scattered_ray,
        })
    }
}

// ---------------------------------------------------------------------------

pub struct Metal {
    pub albedo: Vec3,
}

// ---------------------------------------------------------------------------

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, _rng: &mut ThreadRng) -> Option<MaterialHit> {
        let reflected_direction = Vec3::reflect(&ray_in.direction, &hit_record.normal);

        let attenuation = self.albedo;
        let scattered_ray = Ray {
            origin: hit_record.point,
            direction: reflected_direction,
        };

        Some(MaterialHit {
            attenuation,
            scattered_ray,
        })
    }
}
