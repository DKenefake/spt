use crate::hit_record::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::types::Color;
use crate::utility::{random_double, reflect, refract, sample_lambertian_scatter, sample_unit_vector};
use smolprng::{JsfLarge, PRNG};

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, rec: &HitRecord, prng: &mut PRNG<JsfLarge>) -> (bool, Ray, Color) {
        let scatter_direction = sample_lambertian_scatter(&rec.normal, prng);
        let scattered = Ray {
            origin: rec.p,
            direction: scatter_direction.normalize(),
        };
        let attenuation = self.albedo;
        (true, scattered, attenuation)
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord, prng: &mut PRNG<JsfLarge>) -> (bool, Ray, Color) {
        let mut reflected = r.direction.reflect(rec.normal);
        reflected = (reflected.normalize() + self.fuzz * sample_unit_vector(prng)).normalize();

        let scattered = Ray {
            origin: rec.p,
            direction: reflected,
        };
        let attenuation = self.albedo;

        (
            rec.normal.dot(scattered.direction) > 0.0,
            scattered,
            attenuation,
        )
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
}

impl Material for Dielectric {
    fn scatter(&self, _r: &Ray, rec: &HitRecord, prng: &mut PRNG<JsfLarge>) -> (bool, Ray, Color) {
        let attenuation = Color::ONE;

        let ri = if rec.is_front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_dir = _r.direction.normalize();

        let cos_theta = rec.normal.dot(-unit_dir).min(1.0);
        let sin_theta = cos_theta.mul_add(-cos_theta, 1.0).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let is_reflect = self.reflectance(cos_theta) > random_double(prng);

        let direction =  if cannot_refract || is_reflect{
            reflect(&unit_dir, &rec.normal)
        }else{
            refract(&unit_dir, &rec.normal, ri)
        };

        let scattered = Ray {
            origin: rec.p,
            direction,
        };

        (true, scattered, attenuation)
    }
}
