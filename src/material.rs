use crate::hit_record::HitRecord;
use crate::pdf::PDF;
use crate::ray::Ray;
use crate::types::{Color, P3};
use smolprng::{JsfLarge, PRNG};

pub enum ScatterRay {
    Specular {
        specular_ray: Ray,
        attenuation: Color,
    },
    Scatter {
        pdf: PDF,
        attenuation: Color,
    },
}

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        _r: &Ray,
        _rec: &HitRecord,
        _prng: &mut PRNG<JsfLarge>,
    ) -> Option<ScatterRay> {
        None
    }

    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _b: f64, _p: &P3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn scattering_pdf(&self, _r: &Ray, _scattered: &Ray, _rec: &HitRecord) -> f64 {
        1.0
    }
}
