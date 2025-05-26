use std::sync::Arc;
use crate::aabb::AABB;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::types::{P3, V3};

struct Quad{
    q: P3,
    u: V3,
    v: V3,
    w: V3,
    mat: Arc<dyn Material>,
    aabb: AABB,
    normal: V3,
    d: f64
}

impl Quad{

    pub fn new(q: P3, u: V3, v: V3, mat: Arc<dyn Material>) -> Self{
        let n = u.cross(v);
        let normal = n.normalize();
        let d = normal.dot(q);
        let aabb = Self::set_bounding_box(q, u, v);
        let w = n / (n.dot(n));
        Self{q, u, v, w, mat, aabb, normal, d}
    }

    fn set_bounding_box(q: V3, u: V3, v:V3) -> AABB{
        let diag_1 = AABB::from_points(q, q + u + v);
        let diag_2 = AABB::from_points(q + u, q + v);
        AABB::from_aabbs(&diag_1, &diag_2)
    }

}

impl Hittable for Quad{
    fn hit(&self, r: &Ray, i: &Interval) -> Option<HitRecord> {
        let denom = self.normal.dot(r.direction);

        // if we are parrallel we never hit
        if denom.abs() <= 1E-12{
            return None;
        }

        let t = (self.d - self.normal.dot(r.origin)) / denom;

        // if we hit it but outside of where we care about
        if !i.contains(t){
            return None;
        }

        // see if it is inside of the plane
        let p = r.at(t);

        let planar_hitpt_vector = p - self.q;
        let alpha = self.w.dot(planar_hitpt_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hitpt_vector));

        if 0.0 > alpha || alpha > 1.0{
            return None;
        }

        if 0.0 > beta || beta > 1.0{
            return None;
        }

        let mut hr = HitRecord::from(p, self.normal, t, alpha, beta, self.mat.clone(), true);
        hr.set_face_normal(r, &self.normal);

        Some(hr)
    }

    fn bounding_box(&self) -> AABB {
        self.aabb
    }
}