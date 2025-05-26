use crate::hit_record::HitRecord;
use crate::ray::Ray;
use crate::types::{Color, P3};
use smolprng::{JsfLarge, PRNG};

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        _r: &Ray,
        _rec: &HitRecord,
        _prng: &mut PRNG<JsfLarge>,
    ) -> Option<(Ray, Color)> {
        None
    }

    fn emitted(&self, _u: f64, _b: f64, _p: &P3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}
