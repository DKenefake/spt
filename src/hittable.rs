use crate::aabb::AABB;
use crate::hit_record::HitRecord;
use crate::interval::Interval;
use crate::ray::Ray;

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, i: &Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> AABB;
}


pub(crate) struct CannotHit{

}

impl CannotHit{
    pub fn new() -> Self{
        Self{}
    }
}

impl Hittable for CannotHit {
    fn hit(&self, r: &Ray, i: &Interval) -> Option<HitRecord> {
        None
    }

    fn bounding_box(&self) -> AABB {
        AABB::new()
    }
}