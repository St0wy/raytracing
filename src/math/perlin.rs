use crate::math::vec3::Vec3;
use rand::Rng;

pub const POINT_COUNT: usize = 256;

#[derive(Debug, Clone)]
pub struct Perlin {
    rand_float: [f32; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        let mut rand_float = [0.0f32; POINT_COUNT];
        for i in 0..POINT_COUNT {
            rand_float[i] = rng.gen();
        }

        Self {
            rand_float,
            perm_x: Perlin::generate_perm(),
            perm_y: Perlin::generate_perm(),
            perm_z: Perlin::generate_perm(),
        }
    }

    pub fn noise(&self, p: &Vec3) -> f32 {
        let i = (4.0 * p.x) as i32 & 255;
        let j = (4.0 * p.y) as i32 & 255;
        let k = (4.0 * p.z) as i32 & 255;

        let index = (self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize;
        self.rand_float[index]
    }

    fn generate_perm() -> [i32; POINT_COUNT] {
        let mut p = [0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            p[i] = i as i32;
        }

        Perlin::permute(&mut p);

        p
    }

    fn permute(p: &mut [i32; POINT_COUNT]) {
        let mut rng = rand::thread_rng();
        for i in (0..p.len()).rev() {
            let target = if i == 0 { 0 } else { rng.gen_range(0..i) };
            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;
        }
    }
}
