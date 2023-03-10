use crate::math::vec3::Vec3;
use rand::Rng;

pub const POINT_COUNT: usize = 256;

#[derive(Debug, Clone)]
pub struct Perlin {
    rand_vec: [Vec3; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut rand_vec = [Vec3::zero(); POINT_COUNT];
        for i in 0..POINT_COUNT {
            rand_vec[i] = Vec3::random_range(-1.0..1.0).to_unit();
        }

        Self {
            rand_vec,
            perm_x: Perlin::generate_perm(),
            perm_y: Perlin::generate_perm(),
            perm_z: Perlin::generate_perm(),
        }
    }

    pub fn noise(&self, p: Vec3) -> f32 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c = [[[Vec3::zero(); 2]; 2]; 2];

        for di in 0..2usize {
            for dj in 0..2usize {
                for dk in 0..2usize {
                    let i = (i + di as i32) & 255;
                    let j = (j + dj as i32) & 255;
                    let k = (k + dk as i32) & 255;

                    let index = (self.perm_x[i as usize]
                        ^ self.perm_y[j as usize]
                        ^ self.perm_z[k as usize]) as usize;
                    c[di][dj][dk] = self.rand_vec[index];
                }
            }
        }

        Perlin::perlin_interpolation(c, u, v, w)
    }

    pub fn turbulence(&self, point: Vec3, depth: Option<i32>) -> f32 {
        let depth = depth.unwrap_or(7);

        let mut accumulator = 0.0;
        let mut temp_point = point.clone();
        let mut weight = 1.0;

        for _ in 0..depth {
            accumulator += weight * self.noise(temp_point);
            weight *= 0.5;
            temp_point *= 2.0;
        }

        accumulator.abs()
    }

    fn perlin_interpolation(c: [[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accumulator = 0.0;

        for i in 0..2 {
            let i_f = i as f32;
            for j in 0..2 {
                let j_f = j as f32;
                for k in 0..2 {
                    let k_f = k as f32;
                    let weight_v = Vec3::new(u - i_f, v - j_f, w - k_f);
                    accumulator += (i_f * uu + (1.0 - i_f) * (1.0 - uu))
                        * (j_f * vv + (1.0 - j_f) * (1.0 - vv))
                        * (k_f * ww + (1.0 - k_f) * (1.0 - ww))
                        * c[i][j][k].dot(&weight_v);
                }
            }
        }

        accumulator
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
