use crate::aabb::AABB;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::types::{P3, V3};
use smolprng::{JsfLarge, PRNG};
use std::sync::Arc;

pub struct Quad {
    q: P3,
    u: V3,
    v: V3,
    w: V3,
    mat: Arc<dyn Material>,
    aabb: AABB,
    normal: V3,
    d: f64,
    area: f64,
}

impl Quad {
    pub fn new(q: P3, u: V3, v: V3, mat: Arc<dyn Material>) -> Self {
        let n = u.cross(v);
        let area = n.length();
        let normal = n.normalize();
        let d = normal.dot(q);
        let aabb = Self::set_bounding_box(q, u, v);
        let w = n / (n.dot(n));
        Self {
            q,
            u,
            v,
            w,
            mat,
            aabb,
            normal,
            d,
            area,
        }
    }

    fn set_bounding_box(q: V3, u: V3, v: V3) -> AABB {
        let diag_1 = AABB::from_points(q, q + u + v);
        let diag_2 = AABB::from_points(q + u, q + v);
        AABB::from_aabbs(&diag_1, &diag_2)
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, i: &Interval, _prng: &mut PRNG<JsfLarge>) -> Option<HitRecord> {
        let denom = self.normal.dot(r.direction);

        // if we are parallel we never hit
        if denom.abs() <= 1E-12 {
            return None;
        }

        let t = (self.d - self.normal.dot(r.origin)) / denom;

        // if we hit it but outside of where we care about
        if !i.contains(t) {
            return None;
        }

        // see if it is inside of the plane
        let p = r.at(t);

        let planar_hitpt_vector = p - self.q;
        let alpha = self.w.dot(planar_hitpt_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hitpt_vector));

        if !(0.0..=1.0).contains(&alpha) {
            return None;
        }

        if !(0.0..=1.0).contains(&beta) {
            return None;
        }

        let mut hr = HitRecord::from(p, self.normal, t, alpha, beta, self.mat.clone(), true);
        hr.set_face_normal(r, &self.normal);

        Some(hr)
    }

    fn bounding_box(&self) -> AABB {
        self.aabb
    }

    fn pdf_value(&self, origin: &P3, dir: &V3, time: f64, prng: &mut PRNG<JsfLarge>) -> f64 {
        let hit = self.hit(
            &Ray {
                origin: *origin,
                direction: *dir,
                time,
            },
            &Interval::casting_default(),
            prng,
        );

        match hit {
            None => 0.0,
            Some(rec) => {
                let dist_sqarted = rec.t * rec.t * dir.length_squared();
                let cosine = rec.normal.dot(*dir) / dir.length();

                dist_sqarted / (cosine * self.area)
            }
        }
    }

    fn random(&self, origin: &P3, _time: f64, prng: &mut PRNG<JsfLarge>) -> V3 {
        let p = self.q + prng.gen_f64() * self.u + prng.gen_f64() * self.v;
        p - origin
    }
}
