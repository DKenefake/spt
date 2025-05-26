use crate::aabb::AABB;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::types::V3;
use smolprng::{JsfLarge, PRNG};
use std::sync::Arc;

struct Translate {
    object: Arc<dyn Hittable>,
    offset: V3,
    aabb: AABB,
}

impl Translate {
    pub fn from(object: Arc<dyn Hittable>, offset: V3) -> Self {
        let aabb = object.bounding_box().shift(offset);
        Self {
            object,
            offset,
            aabb,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, i: &Interval, prng: &mut PRNG<JsfLarge>) -> Option<HitRecord> {
        // move the ray for the offset
        let offset_ray = Ray {
            origin: r.origin - self.offset,
            direction: r.direction,
            time: r.time,
        };

        // do we hit?
        let hr = self.object.hit(&offset_ray, i, prng);

        match hr {
            None => None,
            Some(mut rec) => {
                // move it back forwards
                rec.p += self.offset;
                Some(rec)
            }
        }
    }

    fn bounding_box(&self) -> AABB {
        self.aabb
    }
}
