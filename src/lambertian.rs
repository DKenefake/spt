use crate::hit_record::HitRecord;
use crate::material::{Material, ScatterRay};
use crate::pdf::PDF;
use crate::ray::Ray;
use crate::texture::{SolidColor, Texture};
use crate::types::{Color, P3};
use crate::utility::{random_double, reflect, refract, sample_unit_vector};
use smolprng::{JsfLarge, PRNG};
use std::sync::Arc;

pub struct Lambertian {
    pub tex: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn from_color(c: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor { albedo: c }),
        }
    }

    pub fn from_texture(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, rec: &HitRecord, _prng: &mut PRNG<JsfLarge>) -> Option<ScatterRay> {
        Some(ScatterRay::Scatter {
            pdf: PDF::cosine(&rec.normal),
            attenuation: self.tex.value(rec.u, rec.v, &rec.p),
        })
    }

    fn scattering_pdf(&self, _r: &Ray, scattered: &Ray, rec: &HitRecord) -> f64 {
        let cos_theta = rec.normal.dot(scattered.direction.normalize()).max(0.0);
        cos_theta / std::f64::consts::PI
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord, prng: &mut PRNG<JsfLarge>) -> Option<ScatterRay> {
        let mut reflected = r.direction.normalize().reflect(rec.normal);
        reflected = (reflected.normalize() + self.fuzz * sample_unit_vector(prng)).normalize();

        if reflected.dot(rec.normal) > 0.0 {
            Some(ScatterRay::Specular {
                specular_ray: Ray {
                    origin: rec.p,
                    direction: reflected,
                    time: r.time,
                },
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn reflectance(&self, cos: f64) -> f64 {
        let mut r0 = (1.0 - self.refraction_index) / (1.0 + self.refraction_index);
        r0 = r0 * r0;
        (1.0 - r0).mul_add((1.0 - cos).powi(5), r0)
    }

    pub fn fresnel_reflectance(&self, cos: f64) -> f64 {
        //https://www.photometric.io/blog/improving-schlicks-approximation/
        let mut r0 = (1.0 - self.refraction_index) / (1.0 + self.refraction_index);
        r0 = r0 * r0;
        (1.0 - cos - r0).mul_add((1.0 - cos).powi(4), r0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, rec: &HitRecord, prng: &mut PRNG<JsfLarge>) -> Option<ScatterRay> {
        let attenuation = Color::ONE;

        let ri = if rec.is_front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_dir = r.direction.normalize();

        let cos_theta = rec.normal.dot(-unit_dir).min(1.0);
        let sin_theta = cos_theta.mul_add(-cos_theta, 1.0).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let is_reflect = self.fresnel_reflectance(cos_theta) > random_double(prng);

        let direction = if cannot_refract || is_reflect {
            reflect(&unit_dir, &rec.normal)
        } else {
            refract(&unit_dir, &rec.normal, ri)
        };

        Some(ScatterRay::Specular {
            specular_ray: Ray {
                origin: rec.p,
                direction,
                time: r.time,
            },
            attenuation,
        })
    }
}

pub struct DiffuseLight {
    tex: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn from_texture(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }

    pub fn from_color(c: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor { albedo: c }),
        }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, _r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &P3) -> Color {
        if !rec.is_front_face {
            Color::ZERO
        } else {
            self.tex.value(u, v, p)
        }
    }
}

struct Isotropic {
    tex: Arc<dyn Texture>,
}

impl Material for Isotropic {
    fn scatter(&self, _r: &Ray, rec: &HitRecord, _prng: &mut PRNG<JsfLarge>) -> Option<ScatterRay> {
        Some(ScatterRay::Scatter {
            pdf: PDF::sphere(),
            attenuation: self.tex.value(rec.u, rec.v, &rec.p),
        })
    }

    fn scattering_pdf(&self, _r: &Ray, _scattered: &Ray, _rec: &HitRecord) -> f64 {
        0.25 / std::f64::consts::PI
    }
}
