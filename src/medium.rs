use crate::aabb::AABB;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::types::V3;
use crate::utility::random_log_uniform;
use smolprng::{JsfLarge, PRNG};
use std::sync::Arc;

struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: f64,
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, i: &Interval, prng: &mut PRNG<JsfLarge>) -> Option<HitRecord> {
        let universe = Interval::from(f64::MIN, f64::MAX);

        let hit_1 = self.boundary.hit(r, &universe, prng);

        hit_1.as_ref()?;

        let mut rec_1 = hit_1.unwrap();

        let second_interval = Interval::from(rec_1.t + 0.0000001, f64::MAX);

        let hit_2 = self.boundary.hit(r, &second_interval, prng);

        hit_2.as_ref()?;

        let mut rec_2 = hit_2.unwrap();

        if rec_1.t < i.min {
            rec_1.t = i.min;
        }

        if rec_2.t > i.max {
            rec_2.t = i.max;
        }

        if rec_1.t >= rec_2.t {
            return None;
        }

        if rec_1.t < 0.0 {
            rec_1.t = 0.0;
        }

        let r_length = r.direction.length();
        let dist_inside_bound = (rec_2.t - rec_1.t) * r_length;
        let hit_dist = self.neg_inv_density * random_log_uniform(prng);

        if hit_dist > dist_inside_bound {
            return None;
        }

        let t = rec_1.t + hit_dist / r_length;
        let p = r.at(t);
        let normal = V3::X; // arbitrary
        let is_front_face = true;
        let mat = self.phase_function.clone();

        Some(HitRecord::from(p, normal, t, 0.0, 0.0, mat, is_front_face))
    }

    fn bounding_box(&self) -> AABB {
        self.boundary.bounding_box()
    }
}
