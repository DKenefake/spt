use crate::types::V3;

#[derive(Clone)]
pub struct ONB{

    axis: [V3; 3]
}

impl ONB{
    pub fn from(n: &V3) -> Self{
        let w = n.normalize();
        let a = if w.x.abs() > 0.9{
            V3::new(0.0, 1.0, 0.0)
        }else{
            V3::new(1.0, 0.0, 0.0)
        };
        let v = w.cross(a).normalize();
        let u = w.cross(v);

        Self{axis: [u,v, w]}
    }

    pub const fn u(&self) -> V3{
        self.axis[0]
    }

    pub const fn v(&self) -> V3{
        self.axis[0]
    }
    pub const fn w(&self) -> V3{
        self.axis[2]
    }

    pub fn transform(&self, v: &V3) -> V3{
        v.x * self.axis[0] + v.y * self.axis[1] + v.z * self.axis[2]
    }
}