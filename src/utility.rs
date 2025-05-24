use crate::types::V3;
use smolprng::PRNG;
use smolprng::algorithms::JsfLarge;

pub fn make_prng_default() -> PRNG<JsfLarge> {
    PRNG {
        generator: JsfLarge::default(),
    }
}

pub fn random_double(prng: &mut PRNG<JsfLarge>) -> f64 {
    prng.gen_f64()
}

pub fn random_double_in_range(prng: &mut PRNG<JsfLarge>, min: f64, max: f64) -> f64 {
    prng.gen_f64().mul_add(max - min, min)
}

pub fn sample_square(prng: &mut PRNG<JsfLarge>) -> V3 {
    V3::new(random_double(prng) - 0.5, random_double(prng) - 0.5, 0.0)
}

pub fn sample_unit_vector(prng: &mut PRNG<JsfLarge>) -> V3 {
    loop {
        let s = V3::new(prng.normal(), prng.normal(), prng.normal());
        if s.length_squared() >= 1e-80 {
            return s.normalize();
        }
    }
}

pub fn sample_unit_vector_on_hemisphere(normal: &V3, prng: &mut PRNG<JsfLarge>) -> V3 {
    let unit_vector = sample_unit_vector(prng);
    if normal.dot(unit_vector) >= 0.0 {
        unit_vector
    } else {
        -unit_vector
    }
}

pub fn sample_lambertian_scatter(normal: &V3, prng: &mut PRNG<JsfLarge>) -> V3 {
    // sample from the lambertian brdf but in a wat that doesn't brick the numerics
    let small_numbers = 1e-8;

    let sampled_point = sample_unit_vector_on_hemisphere(normal, prng);

    if sampled_point.length_squared() <= small_numbers {
        *normal
    }else{
        sampled_point
    }
}

pub fn linear_to_gamma(x: f64) -> f64 {
    if x > 0.0 { x.sqrt() } else { 0.0 }
}


pub fn reflect(v_in: &V3, normal: &V3) -> V3{
    (v_in - 2.0 * v_in.dot(*normal) * normal).normalize()
}

pub fn refract(uv: &V3, normal: &V3, ri: f64) -> V3 {
    let cos_theta = normal.dot(-uv).min(1.0);
    let r_out_perp = ri * (uv + cos_theta * normal);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * normal;
    (r_out_perp + r_out_parallel).normalize()
}
