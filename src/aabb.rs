use crate::interval::Interval;
use crate::ray::Ray;
use crate::types::P3;

#[derive(Copy, Clone)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub const fn new() -> Self {
        Self {
            x: Interval::new(),
            y: Interval::new(),
            z: Interval::new(),
        }
    }

    pub const fn from(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_points(a: P3, b: P3) -> Self {
        let x = if a[0] <= b[0] {
            Interval::from(a[0], b[0])
        } else {
            Interval::from(b[0], a[0])
        };
        let y = if a[1] <= b[1] {
            Interval::from(a[1], b[1])
        } else {
            Interval::from(b[1], a[1])
        };
        let z = if a[2] <= b[2] {
            Interval::from(a[2], b[2])
        } else {
            Interval::from(b[2], a[2])
        };

        Self { x, y, z }
    }

    pub const fn from_aabbs(a: &Self, b: &Self) -> Self {
        Self {
            x: Interval::from_intervals(&a.x, &b.x),
            y: Interval::from_intervals(&a.y, &b.y),
            z: Interval::from_intervals(&a.z, &b.z),
        }
    }

    pub const fn get_axis_interval(&self, n: usize) -> Interval {
        match n {
            0 => self.x,
            1 => self.y,
            _ => self.z,
        }
    }

    pub fn longest_axis(&self) -> usize{
        if self.x.size() > self.y.size(){
            if self.x.size() > self.z.size(){
                0
            }else{
                2
            }
        }else{
            if self.y.size() > self.z.size(){
                1
            }else{
                2
            }
        }
    }

    pub fn hit(&self, r: &Ray, i: &Interval) -> Option<Interval> {
        let mut hit_width = *i;

        for axis in 0..3 {
            let ax = self.get_axis_interval(axis);

            let ad_inv = 1.0 / r.direction[axis];

            let (t0, t1) = {
                let t0 = (ax.min - r.origin[axis]) * ad_inv;
                let t1 = (ax.max - r.origin[axis]) * ad_inv;

                if ad_inv < 0.0 { (t1, t0) } else { (t0, t1) }
            };

            hit_width.min = t0.max(hit_width.min);
            hit_width.max = t1.min(hit_width.max);

            if hit_width.max < hit_width.min {
                return None;
            }
        }

        Some(hit_width)
    }
}

#[cfg(test)]
mod tests{

    use crate::aabb::AABB;
    use crate::ray::Ray;
    use crate::interval::Interval;
    use crate::types::{P3, V3};

    #[test]
    fn check_unit_ray_unit_box(){
        let ray = Ray::from(&P3::new(-10.0, 0.0, 0.0), &V3::X, 0.0);

        let mins = -V3::ONE;
        let maxs = V3::ONE;
        let aabb = AABB::from_points(mins, maxs);

        let does_hit = aabb.hit(&ray, &Interval::from(f64::MIN, f64::MAX));

        match does_hit {
            None => assert_eq!(0, 1),
            Some(x) => {
                assert_eq!(x.min, 9.0);
                assert_eq!(x.max, 11.0);
            }
        }
    }

}
