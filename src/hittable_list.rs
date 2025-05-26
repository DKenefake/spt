use crate::aabb::AABB;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use smolprng::{JsfLarge, PRNG};

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, i: &Interval, prng: &mut PRNG<JsfLarge>) -> Option<HitRecord> {
        fn closest_hit_record(hr1: Option<HitRecord>, hr2: HitRecord) -> HitRecord {
            match hr1 {
                Some(hr) => {
                    if hr.t >= hr2.t {
                        hr2
                    } else {
                        hr
                    }
                }
                None => hr2,
            }
        }

        let mut hr = None;

        for obj in self.objects.iter() {
            let obj_hr = obj.hit(r, i, prng);

            match obj_hr {
                None => {}
                Some(hr2) => hr = Some(closest_hit_record(hr, hr2)),
            }
        }
        hr
    }

    fn bounding_box(&self) -> AABB {
        let mut list_aabb = AABB::new();

        for obj in &self.objects {
            list_aabb = AABB::from_aabbs(&list_aabb, &obj.bounding_box());
        }

        list_aabb
    }
}
