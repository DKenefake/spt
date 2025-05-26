use crate::aabb::AABB;
use crate::hit_record::HitRecord;
use crate::interval::Interval;
use crate::ray::Ray;

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, i: &Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> AABB;
}
