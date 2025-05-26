use crate::aabb::AABB;
use crate::hit_record::HitRecord;
use crate::hittable::{Hittable, CannotHit};
use crate::interval::Interval;
use crate::ray::Ray;
use std::cmp::Ordering;
use std::sync::Arc;

pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bounding_box: AABB,
}

impl BVHNode {
    pub fn from_two_hittables(obj1: Arc<dyn Hittable>, obj2: Arc<dyn Hittable>) -> Self {
        let aabb = AABB::from_aabbs(&obj1.bounding_box(), &obj2.bounding_box());
        Self {
            left: obj1,
            right: obj2,
            bounding_box: aabb,
        }
    }

    pub fn from(obj_list: &mut Vec<Box<dyn Hittable>>) -> Self {
        Self::make_level(obj_list, 0, obj_list.len())
    }

    pub fn make_level(
        obj_list: &mut Vec<Box<dyn Hittable>>,
        start: usize,
        end: usize,
    ) -> Self {

        let mut aabb = AABB::new();

        for obj in obj_list.iter(){
            aabb = AABB::from_aabbs(&aabb, &obj.bounding_box());
        }

        let axis = aabb.longest_axis();

        let object_span = end - start;

        let (left, right) = match object_span {
            1 => {
                let first: Arc<dyn Hittable> = obj_list.remove(0).into();
                let second: Arc<dyn Hittable> = Arc::new(CannotHit::new());
                (first, second)
            }
            2 => {
                let first: Arc<dyn Hittable> = obj_list.remove(0).into();
                let second: Arc<dyn Hittable> = obj_list.remove(0).into();

                match Self::hittable_compare(&*first, &*second, axis) {
                    Ordering::Less => (first, second),
                    _ => (second, first),
                }
            }
            _ => {
                obj_list.sort_by(|x, y| Self::hittable_compare(&**x, &**y, axis));
                let mid = start + object_span / 2;

                let left: Arc<dyn Hittable> =
                    Arc::new(Self::make_level(obj_list, start, mid));
                let right: Arc<dyn Hittable> =
                    Arc::new(Self::make_level(obj_list, mid, end));
                (left, right)
            }
        };

        Self {
            left,
            right,
            bounding_box: aabb,
        }
    }

    pub fn hittable_compare(a: &dyn Hittable, b: &dyn Hittable, axis: usize) -> Ordering {
        let box_a = a.bounding_box();
        let box_b = b.bounding_box();

        let x = box_a.get_axis_interval(axis).min;

        let y = box_b.get_axis_interval(axis).min;

        if x < y {
            Ordering::Less
        } else if x > y {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, i: &Interval) -> Option<HitRecord> {

        if !self.bounding_box.hit(r, i) {
            return None;
        }

        let hit_left = self.left.hit(r, i);

        let t = if let Some(record) = hit_left.clone() {
            record.t
        } else {
            i.max
        };

        let hit_right = self.right.hit(r, &Interval::from(i.min, t));

        hit_right.or(hit_left)
    }

    fn bounding_box(&self) -> AABB {
        self.bounding_box
    }
}
