use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
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
    fn hit(&self, r: &Ray, i: &Interval) -> Option<HitRecord> {
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
            let obj_hr = obj.hit(r, i);

            match obj_hr {
                None => {}
                Some(hr2) => hr = Some(closest_hit_record(hr, hr2)),
            }
        }
        hr
    }
}
