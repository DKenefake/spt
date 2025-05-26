use crate::hit_record::HitRecord;
use crate::ray::Ray;
use crate::types::{Color, P3};
use smolprng::{JsfLarge, PRNG};

pub trait Material: Send + Sync {
    fn scatter(&self, r: &Ray, rec: &HitRecord, prng: &mut PRNG<JsfLarge>) -> (bool, Ray, Color);

    fn emitted(&self, u: f64, b: f64, p: &P3) -> Color{
        Color::new(0.0, 0.0, 0.0)
    }
}
