use crate::hit_record::HitRecord;
use crate::ray::Ray;
use crate::interval::Interval;

pub trait Hittable{

    fn hit(&self, r: &Ray, i: &Interval) -> Option<HitRecord>;

}
