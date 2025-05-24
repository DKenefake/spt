use smolprng::{JsfLarge, PRNG};
use crate::hit_record::HitRecord;
use crate::ray::Ray;
use crate::types::{Color};

pub trait Material{
    fn scatter(&self, r: &Ray, rec: &HitRecord, prng: &mut PRNG<JsfLarge>) -> (bool, Ray, Color);
}