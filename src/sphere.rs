use crate::aabb::AABB;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::lambertian::Lambertian;
use crate::material::Material;
use crate::ray::Ray;
use crate::types::{Color, P3, V3};
use std::sync::Arc;

#[derive(Clone)]
pub struct Sphere {
    pub center: Ray,
    pub radius: f64,
    pub mat: Arc<dyn Material>,
    pub aabb: AABB,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            center: Ray::new(),
            radius: 1.0,
            mat: Arc::new(Lambertian {
                albedo: Color::new(0.5, 0.5, 0.5),
            }),
            aabb: AABB::from_points(-P3::ONE, P3::ONE),
        }
    }

    pub fn from(center: Ray, radius: f64, mat: Arc<dyn Material>) -> Self {
        let r_vec = V3::splat(radius);

        let aabb1 = AABB::from_points(center.at(0.0) - r_vec, center.at(0.0) + r_vec);
        let aabb2 = AABB::from_points(center.at(1.0) - r_vec, center.at(1.0) + r_vec);

        Self {
            center,
            radius,
            mat,
            aabb: AABB::from_aabbs(&aabb1, &aabb2),
        }
    }

    pub fn static_sphere(center: P3, radius: f64, mat: Arc<dyn Material>) -> Self {
        let r_vec = V3::splat(radius);

        Self {
            center: Ray {
                origin: center,
                direction: V3::ZERO,
                time: 0.0f64,
            },
            radius,
            mat,
            aabb: AABB::from_points(center - r_vec, center + r_vec),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, i: &Interval) -> Option<HitRecord> {
        let current_center = self.center.at(r.time);

        let oc = current_center - r.origin;

        let a = r.direction.length_squared();
        let h = r.direction.dot(oc);
        let c = self.radius.mul_add(-self.radius, oc.length_squared());

        let discriminant = h.mul_add(h, -a * c);

        if discriminant <= 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let a_inv = 1.0 / a;

        let mut root = (h - sqrtd) * a_inv;

        if !i.surrounds(root) {
            root = (h + sqrtd) * a_inv;
            if !i.surrounds(root) {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - current_center) / self.radius;
        let mut hr = HitRecord::from(p, outward_normal, root, self.mat.clone(), true);
        hr.set_face_normal(r, &outward_normal);

        Some(hr)
    }

    fn bounding_box(&self) -> AABB {
        self.aabb
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}
