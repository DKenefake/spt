use crate::aabb::AABB;
use crate::hit_record::HitRecord;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::types::{P3, V3};
use smolprng::{JsfLarge, PRNG};

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, i: &Interval, prng: &mut PRNG<JsfLarge>) -> Option<HitRecord>;

    fn bounding_box(&self) -> AABB;

    fn pdf_value(&self, _origin: &P3, _dir: &V3, _time: f64, _prng: &mut PRNG<JsfLarge>) -> f64 {
        0.0
    }

    fn random(&self, _origin: &P3, _time: f64, _prng: &mut PRNG<JsfLarge>) -> V3 {
        V3::X
    }
}
