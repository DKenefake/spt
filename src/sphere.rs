use crate::aabb::AABB;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::lambertian::Lambertian;
use crate::material::Material;
use crate::onb::ONB;
use crate::ray::Ray;
use crate::types::{Color, P3, V3};
use smolprng::{JsfLarge, PRNG};
use std::sync::Arc;

#[derive(Clone)]
pub struct Sphere {
    pub center: Ray,
    pub radius: f64,
    pub mat: Arc<dyn Material>,
    pub aabb: AABB,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            center: Ray::new(),
            radius: 1.0,
            mat: Arc::new(Lambertian::from_color(Color::new(0.5, 0.5, 0.5))),
            aabb: AABB::from_points(-P3::ONE, P3::ONE),
        }
    }

    pub fn from(center: Ray, radius: f64, mat: Arc<dyn Material>) -> Self {
        let r_vec = V3::splat(radius);

        let aabb1 = AABB::from_points(center.at(0.0) - r_vec, center.at(0.0) + r_vec);
        let aabb2 = AABB::from_points(center.at(1.0) - r_vec, center.at(1.0) + r_vec);

        Self {
            center,
            radius,
            mat,
            aabb: AABB::from_aabbs(&aabb1, &aabb2),
        }
    }

    pub fn static_sphere(center: P3, radius: f64, mat: Arc<dyn Material>) -> Self {
        let r_vec = V3::splat(radius);

        Self {
            center: Ray {
                origin: center,
                direction: V3::ZERO,
                time: 0.0f64,
            },
            radius,
            mat,
            aabb: AABB::from_points(center - r_vec, center + r_vec),
        }
    }

    fn get_sphere_uv(p: &P3) -> (f64, f64) {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + std::f64::consts::PI;

        let u = phi / (2.0 * std::f64::consts::PI);
        let v = theta / std::f64::consts::PI;
        (u, v)
    }

    fn random_to_sphere(rad: f64, dist_sqrd: f64, prng: &mut PRNG<JsfLarge>) -> V3 {
        let r1 = prng.gen_f64();
        let r2 = prng.gen_f64();

        let z = r2.mul_add((1.0 - rad * rad / dist_sqrd).sqrt() - 1.0, 1.0);
        let phi = 2.0 * std::f64::consts::PI * r1;

        let (phi_sin, phi_cos) = phi.sin_cos();
        let z_sqrt = z.mul_add(-z, 1.0).max(0.0);
        let x = phi_cos * z_sqrt;
        let y = phi_sin * z_sqrt;

        V3::new(x, y, z)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, i: &Interval, _prng: &mut PRNG<JsfLarge>) -> Option<HitRecord> {
        let current_center = self.center.at(r.time);

        let oc = current_center - r.origin;

        let a = r.direction.length_squared();
        let h = r.direction.dot(oc);
        let c = self.radius.mul_add(-self.radius, oc.length_squared());

        let discriminant = h.mul_add(h, -a * c);

        if discriminant <= 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let a_inv = 1.0 / a;

        let mut root = (h - sqrtd) * a_inv;

        if !i.surrounds(root) {
            root = (h + sqrtd) * a_inv;
            if !i.surrounds(root) {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - current_center) / self.radius;
        let mut hr = HitRecord::from(p, outward_normal, root, 0.0, 0.0, self.mat.clone(), true);

        let (u, v) = Self::get_sphere_uv(&p);

        hr.u = u;
        hr.v = v;

        hr.set_face_normal(r, &outward_normal);

        Some(hr)
    }

    fn bounding_box(&self) -> AABB {
        self.aabb
    }

    fn pdf_value(&self, origin: &P3, dir: &V3, time: f64, prng: &mut PRNG<JsfLarge>) -> f64 {
        // only works for stationary spheres!

        let hit_rec = self.hit(
            &Ray {
                origin: *origin,
                direction: *dir,
                time,
            },
            &Interval::casting_default(),
            prng,
        );

        if hit_rec.is_none() {
            return 0.0;
        }

        let dist_squared = (self.center.at(time) - origin).length_squared();
        let cos_theta_max = (1.0 - self.radius * self.radius / dist_squared)
            .max(0.0)
            .sqrt();
        let solid_angle = 2.0 * std::f64::consts::PI * (1.0 - cos_theta_max);

        1.0 / solid_angle
    }

    fn random(&self, origin: &P3, time: f64, prng: &mut PRNG<JsfLarge>) -> V3 {
        let dir = self.center.at(time) - origin;
        let dist_squared = dir.length_squared();
        let uvw = ONB::from(&dir);

        uvw.transform(&Self::random_to_sphere(self.radius, dist_squared, prng))
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}
