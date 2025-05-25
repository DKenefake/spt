use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::lambertian::Lambertian;
use crate::material::Material;
use crate::ray::Ray;
use crate::types::{Color, P3};
use std::sync::Arc;

pub struct Sphere {
    pub center: P3,
    pub radius: f64,
    pub mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            center: P3::ZERO,
            radius: 1.0,
            mat: Arc::new(Lambertian {
                albedo: Color::new(0.5, 0.5, 0.5),
            }),
        }
    }

    pub fn from(center: P3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, i: &Interval) -> Option<HitRecord> {
        let oc = self.center - r.origin;

        let a = r.direction.length_squared();
        let h = r.direction.dot(oc);
        let c = self.radius.mul_add(-self.radius, oc.length_squared());

        let discriminant = h.mul_add(h, -a * c);

        if discriminant <= 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;

        if !i.surrounds(root) {
            root = (h + sqrtd) / a;
            if !i.surrounds(root) {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;
        let mut hr = HitRecord::from(p, outward_normal, root, self.mat.clone(), true);
        hr.set_face_normal(r, &outward_normal);

        Some(hr)
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}
