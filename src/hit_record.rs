use std::rc::Rc;
use crate::ray::Ray;
use crate::types::{P3, V3};
use crate::material::Material;

pub struct HitRecord{
    pub(crate) p: P3,
    pub normal: V3,
    pub material: Rc<dyn Material>,
    pub(crate) t: f64,
    pub(crate) is_front_face: bool
}

impl HitRecord{
    pub fn from(p: P3, normal: V3, t: f64, material: Rc<dyn Material>, is_front_face:bool) -> Self{
        Self{p, normal, material, t, is_front_face}
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &V3){
        self.is_front_face = r.direction.dot(*outward_normal) <= 0.0;
        self.normal = if self.is_front_face { *outward_normal } else {-outward_normal};
    }
}