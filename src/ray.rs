use crate::types::{P3, V3};

pub struct Ray {
    pub origin: P3,
    pub direction: V3,
    pub time: f64
}

impl Ray {
    pub const fn new() -> Self {
        Self {
            origin: P3::ZERO,
            direction: V3::X,
            time: 0.0,
        }
    }

    pub const fn from(origin: &P3, direction: &V3, time: f64) -> Self {
        Self {
            origin: *origin,
            direction: *direction,
            time
        }
    }

    pub fn at(&self, t: f64) -> V3 {
        self.origin + t * self.direction
    }
}

impl Default for Ray {
    fn default() -> Self {
        Self::new()
    }
}
