use crate::interval::Interval;
use crate::ray::Ray;
use crate::types::P3;

struct AABB{
    x: Interval,
    y: Interval,
    z: Interval
}

impl AABB {

    pub fn new() -> Self{
        Self{x: Interval::new(), y: Interval::new(), z: Interval::new()}
    }

    pub fn from(x: Interval, y: Interval, z: Interval) -> Self{
        Self{x, y, z}
    }

    pub fn from_points(a: P3, b: P3) -> Self{
        let mins = a.min(b);
        let maxs = a.max(b);

        Self{x: Interval::from(mins.x, maxs.x), y: Interval::from(mins.y, maxs.y),z: Interval::from(mins.z, maxs.z), }
    }

    pub fn get_axis_interval(&self, n: usize) -> Interval{
        match n {
            0 => self.x,
            1 => self.y,
            _ => self.z
        }
    }
}