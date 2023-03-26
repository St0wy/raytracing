use crate::math::vec3::Vec3Ext;
use glam::Vec3A;
use rand::Rng;

pub const POINT_COUNT: usize = 256;

#[derive(Debug, Clone)]
pub struct Perlin {
    rand_vec: [Vec3A; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut rand_vec = [Vec3A::ZERO; POINT_COUNT];
        for vec in rand_vec.iter_mut() {
            *vec = Vec3A::random_range(-1.0..1.0).normalize();
        }

        Self {
            rand_vec,
            perm_x: Perlin::generate_perm(),
            perm_y: Perlin::generate_perm(),
            perm_z: Perlin::generate_perm(),
        }
    }

    pub fn noise(&self, p: Vec3A) -> f32 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c = [[[Vec3A::ZERO; 2]; 2]; 2];

        for (di, ci) in c.iter_mut().enumerate() {
            for (dj, cj) in ci.iter_mut().enumerate() {
                for (dk, ck) in cj.iter_mut().enumerate() {
                    let i = (i + di as i32) & 255;
                    let j = (j + dj as i32) & 255;
                    let k = (k + dk as i32) & 255;

                    let index = (self.perm_x[i as usize]
                        ^ self.perm_y[j as usize]
                        ^ self.perm_z[k as usize]) as usize;
                    *ck = self.rand_vec[index];
                }
            }
        }

        Perlin::perlin_interpolation(c, u, v, w)
    }

    pub fn turbulence(&self, point: Vec3A, depth: Option<i32>) -> f32 {
        let depth = depth.unwrap_or(7);

        let mut accumulator = 0.0;
        let mut temp_point = point;
        let mut weight = 1.0;

        for _ in 0..depth {
            accumulator += weight * self.noise(temp_point);
            weight *= 0.5;
            temp_point *= 2.0;
        }

        accumulator.abs()
    }

    fn perlin_interpolation(c: [[[Vec3A; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accumulator = 0.0;

        for (i, ci) in c.iter().enumerate() {
            let i_f = i as f32;
            for (j, cj) in ci.iter().enumerate() {
                let j_f = j as f32;
                for (k, ck) in cj.iter().enumerate() {
                    let k_f = k as f32;
                    let weight_v = Vec3A::new(u - i_f, v - j_f, w - k_f);
                    accumulator += (i_f * uu + (1.0 - i_f) * (1.0 - uu))
                        * (j_f * vv + (1.0 - j_f) * (1.0 - vv))
                        * (k_f * ww + (1.0 - k_f) * (1.0 - ww))
                        * ck.dot(weight_v);
                }
            }
        }

        accumulator
    }

    fn generate_perm() -> [i32; POINT_COUNT] {
        let mut p = [0; POINT_COUNT];
        for (index, elem) in p.iter_mut().enumerate() {
            *elem = index as i32;
        }

        Perlin::permute(&mut p);

        p
    }

    fn permute(p: &mut [i32; POINT_COUNT]) {
        let mut rng = rand::thread_rng();
        for i in (0..p.len()).rev() {
            let target = if i == 0 { 0 } else { rng.gen_range(0..i) };
            p.swap(i, target);
        }
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}
