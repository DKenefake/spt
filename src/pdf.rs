use std::sync::Arc;
use smolprng::{JsfLarge, PRNG};
use crate::hittable::Hittable;
use crate::onb::ONB;
use crate::types::{P3, V3};
use crate::utility::{sample_cosine_direction, sample_unit_vector};

#[derive(Clone)]
pub enum PDF<'a>{
    Sphere {},
    Cosine {uvw: ONB},
    Hittable {obj: &'a Arc<dyn Hittable>, origin: P3},
    Mixture {p : &'a PDF<'a>, q: &'a PDF<'a>}
}

impl<'a> PDF<'a>{
    pub fn cosine(w: &V3) -> Self{
        PDF::Cosine {uvw: ONB::from(w)}
    }

    pub fn hittable(obj: &'a Arc<dyn Hittable>, origin: &P3) -> Self{
        PDF::Hittable {obj, origin: *origin}
    }

    pub fn mixture(p: &'a PDF, q: &'a PDF) -> Self{
        PDF::Mixture {p, q}
    }

    pub fn sphere() -> Self{
        PDF::Sphere {}
    }

    pub fn value(&self, dir: &V3, time: f64, prng: &mut PRNG<JsfLarge>) -> f64{
        match self {
            PDF::Sphere {} => {
                0.25 / std::f64::consts::PI
            }
            PDF::Cosine {uvw } => {
                let cos_theta = dir.normalize().dot(uvw.w());
                (cos_theta / std::f64::consts::PI).max(0.0)
            }
            PDF::Hittable {obj, origin} => {
                obj.pdf_value(&origin, dir, time, prng)
            }
            PDF::Mixture {p, q} => {
                0.5f64 * p.value(dir, time, prng) + 0.5 * q.value(dir, time, prng)
            }
        }
    }

    pub fn generate(&self, time: f64, prng: &mut PRNG<JsfLarge>) -> V3{
        match self {
            PDF::Sphere {} => {
                sample_unit_vector(prng)
            }
            PDF::Cosine {uvw } => {
                uvw.transform(&sample_cosine_direction(prng))
            }
            PDF::Hittable {obj, origin} => {
                obj.random(&origin, time, prng)
            }
            PDF::Mixture {p, q} => {
                if prng.gen_f64() <= 0.5{
                    p.generate(time, prng)
                }else{
                    q.generate(time, prng)
                }
            }
        }
    }
}
