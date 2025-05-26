use crate::interval::Interval;
use crate::ray::Ray;
use crate::types::{P3, V3};

#[derive(Copy, Clone)]
pub struct AABB {
    pub min: V3,
    pub max: V3,
}

impl AABB {
    pub const fn new() -> Self {
        Self {
            min: V3::ZERO,
            max: V3::ONE,
        }
    }

    pub const fn from(x: Interval, y: Interval, z: Interval) -> Self {
        Self {
            min: V3::new(x.min, y.min, z.min),
            max: V3::new(x.max, y.max, z.max),
        }
    }

    pub fn from_points(a: P3, b: P3) -> Self {
        Self {
            min: a.min(b),
            max: a.max(b),
        }
    }

    pub fn from_aabbs(a: &Self, b: &Self) -> Self {
        Self {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }

    pub const fn get_axis_interval(&self, n: usize) -> Interval {
        match n {
            0 => Interval::from(self.min.x, self.max.x),
            1 => Interval::from(self.min.y, self.max.y),
            _ => Interval::from(self.min.z, self.max.z),
        }
    }

    pub fn longest_axis(&self) -> usize {
        let delta = self.max - self.min;
        delta.max_position()
    }

    pub fn hit(&self, r: &Ray, i: &Interval) -> bool {
        let inv_d = r.direction.recip();
        let t0 = (self.min - r.origin) * inv_d;
        let t1 = (self.max - r.origin) * inv_d;

        let t_min = t0.min(t1);
        let t_max = t0.max(t1);

        t_min.max_element().max(i.min) <= t_max.min_element().min(i.max)
    }
}

#[cfg(test)]
mod tests {

    use crate::aabb::AABB;
    use crate::interval::Interval;
    use crate::ray::Ray;
    use crate::types::{P3, V3};

    #[test]
    fn check_unit_ray_unit_box() {
        let ray = Ray::from(&P3::new(-10.0, 0.0, 0.0), &V3::X, 0.0);

        let mins = -V3::ONE;
        let maxs = V3::ONE;
        let aabb = AABB::from_points(mins, maxs);

        let does_hit = aabb.hit(&ray, &Interval::from(f64::MIN, f64::MAX));

        assert_eq!(does_hit, true);
    }
}
