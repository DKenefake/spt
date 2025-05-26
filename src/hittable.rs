use crate::aabb::AABB;
use crate::hit_record::HitRecord;
use crate::interval::Interval;
use crate::ray::Ray;
use smolprng::{JsfLarge, PRNG};

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, i: &Interval, prng: &mut PRNG<JsfLarge>) -> Option<HitRecord>;

    fn bounding_box(&self) -> AABB;
}
