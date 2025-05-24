use crate::types::{P3, V3};

pub struct Ray{
    pub origin: P3,
    pub direction: V3
}

impl Ray{

    pub const fn new() -> Self{
        Self{origin: P3::ZERO, direction: V3::X}
    }

    pub fn from(origin: &P3, direction: &V3) -> Self{
        Self{origin: origin.clone(), direction: direction.clone()}
    }

    pub fn at(&self, t:f64) -> V3{
        self.origin + t * self.direction
    }
}

impl Default for Ray{
    fn default() -> Self {
        Self::new()
    }
}