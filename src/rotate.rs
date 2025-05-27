use crate::aabb::AABB;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::types::{P3, V3};
use smolprng::{JsfLarge, PRNG};
use std::sync::Arc;

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    aabb: AABB,
}

impl RotateY {
    pub fn from(object: Arc<dyn Hittable>, angle: f64) -> Self {
        let rads = angle.to_radians();
        let sin_theta = rads.sin();
        let cos_theta = rads.cos();

        let bbox = object.bounding_box();

        let mut min = P3::splat(f64::MAX);
        let mut max = P3::splat(f64::MIN);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = (i as f64).mul_add(bbox.max.x, (1 - i) as f64 * bbox.min.x);
                    let y = (j as f64).mul_add(bbox.max.y, (1 - j) as f64 * bbox.min.y);
                    let z = (k as f64).mul_add(bbox.max.z, (1 - k) as f64 * bbox.min.z);

                    let new_x = cos_theta.mul_add(x, sin_theta * z);
                    let new_z = (-sin_theta).mul_add(x, cos_theta * z);

                    let tester = V3::new(new_x, y, new_z);

                    min = min.min(tester);
                    max = max.max(tester);
                }
            }
        }

        let aabb = AABB::from_points(min, max);

        Self {
            object,
            sin_theta,
            cos_theta,
            aabb,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, i: &Interval, prng: &mut PRNG<JsfLarge>) -> Option<HitRecord> {
        // move from world space to object space
        let origin = P3::new(
            self.cos_theta
                .mul_add(r.origin.x, -(self.sin_theta * r.origin.z)),
            r.origin.y,
            self.sin_theta
                .mul_add(r.origin.x, self.cos_theta * r.origin.z),
        );
        let direction = V3::new(
            self.cos_theta
                .mul_add(r.direction.x, -(self.sin_theta * r.direction.z)),
            r.direction.y,
            self.sin_theta
                .mul_add(r.direction.x, self.cos_theta * r.direction.z),
        );

        let rotated_ray = Ray {
            origin,
            direction,
            time: r.time,
        };

        // determine if an intersection exits in the object space
        let hit_rec = self.object.hit(&rotated_ray, i, prng);

        match hit_rec {
            None => None,
            Some(mut rec) => {
                // move the intersection back to world space
                rec.p = P3::new(
                    self.cos_theta.mul_add(rec.p.x, self.sin_theta * rec.p.z),
                    rec.p.y,
                    (-self.sin_theta).mul_add(rec.p.x, self.cos_theta * rec.p.z),
                );
                rec.normal = V3::new(
                    self.cos_theta
                        .mul_add(rec.normal.x, self.sin_theta * rec.normal.z),
                    r.origin.y,
                    (-self.sin_theta).mul_add(rec.normal.x, self.cos_theta * rec.normal.z),
                );
                Some(rec)
            }
        }
    }

    fn bounding_box(&self) -> AABB {
        self.aabb
    }
}
