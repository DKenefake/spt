use crate::types::P3;
use smolprng::{JsfLarge, PRNG};

pub struct Perlin {
    random_floats: [f64; 256],
    perm_x: [usize; 256],
    perm_y: [usize; 256],
    perm_z: [usize; 256],
}

impl Perlin {
    pub fn new(prng: &mut PRNG<JsfLarge>) -> Self {
        let mut random_floats = [0.0; 256];
        let mut perm_x = [0; 256];
        let mut perm_y = [0; 256];
        let mut perm_z = [0; 256];

        for i in 0..256 {
            random_floats[i] = prng.gen_f64();
        }

        Self::permute(&mut perm_x, 256, prng);
        Self::permute(&mut perm_y, 256, prng);
        Self::permute(&mut perm_z, 256, prng);

        Self {
            random_floats,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &P3) -> f64 {
        let ijk = (p * 4.0).abs().as_usizevec3() & 255;
        let location = self.perm_x[ijk.x] ^ self.perm_y[ijk.y] ^ self.perm_z[ijk.z];
        self.random_floats[location]
    }

    fn permute(perm: &mut [usize; 256], n: usize, prng: &mut PRNG<JsfLarge>) {
        for i in 0..n {
            let target = prng.gen_u8() as usize;
            perm.swap(i, target);
        }
    }
}
