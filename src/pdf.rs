use crate::hittable::Hittable;
use crate::onb::ONB;
use crate::types::{P3, V3};
use crate::utility::{sample_cosine_direction, sample_unit_vector};
use smolprng::{JsfLarge, PRNG};
use std::sync::Arc;

#[derive(Clone)]
pub enum PDF {
    Sphere {},
    Cosine { uvw: ONB },
    Hittable { obj: Arc<dyn Hittable>, origin: P3 },
}

impl PDF {
    pub fn cosine(w: &V3) -> Self {
        Self::Cosine { uvw: ONB::from(w) }
    }

    pub fn hittable(obj: Arc<dyn Hittable>, origin: &P3) -> Self {
        Self::Hittable {
            obj,
            origin: *origin,
        }
    }

    pub const fn sphere() -> Self {
        Self::Sphere {}
    }

    pub fn value(&self, dir: &V3, time: f64, prng: &mut PRNG<JsfLarge>) -> f64 {
        match self {
            Self::Sphere {} => 0.25 / std::f64::consts::PI,
            Self::Cosine { uvw } => {
                let cos_theta = dir.normalize().dot(uvw.w());
                (cos_theta / std::f64::consts::PI).max(0.0)
            }
            Self::Hittable { obj, origin } => obj.pdf_value(origin, dir, time, prng),
        }
    }

    pub fn generate(&self, time: f64, prng: &mut PRNG<JsfLarge>) -> V3 {
        match self {
            Self::Sphere {} => sample_unit_vector(prng),
            Self::Cosine { uvw } => uvw.transform(&sample_cosine_direction(prng)),
            Self::Hittable { obj, origin } => obj.random(origin, time, prng),
        }
    }
}
