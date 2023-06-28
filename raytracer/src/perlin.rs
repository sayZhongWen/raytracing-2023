use crate::rtweekend::*;
use crate::vec3::Point3;
use crate::Vec3;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ranfloat: Vec<f64>,
    ranvec: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    pub fn new() -> Self {
        let mut ranfloat = Vec::new();
        let mut ranvec = Vec::new();
        for _ in 0..POINT_COUNT {
            ranfloat.push(random_f64());
            ranvec.push(Vec3::random(-1.0, 1.0).unit_vector());
        }
        Self {
            ranfloat,
            ranvec,
            perm_x: Perlin::perlin_generate_perm(),
            perm_y: Perlin::perlin_generate_perm(),
            perm_z: Perlin::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();
        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;
        let mut c: Vec<Vec<Vec<Vec3>>> = vec![
            vec![
                vec![Vec3::zero(), Vec3::zero()],
                vec![Vec3::zero(), Vec3::zero()],
            ],
            vec![
                vec![Vec3::zero(), Vec3::zero()],
                vec![Vec3::zero(), Vec3::zero()],
            ],
        ];
        for (di, value1) in c.iter_mut().enumerate().take(2) {
            for (dj, value2) in value1.iter_mut().enumerate().take(2) {
                for (dk, value3) in value2.iter_mut().enumerate().take(2) {
                    *value3 = self.ranvec[(self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize]
                        .clone();
                }
            }
        }
        Perlin::perlin_interp(&c, u, v, w)
    }
    // fn trilinear_interp(c: &[Vec<Vec<f64>>], u: f64, v: f64, w: f64) -> f64 {
    //     let mut accum = 0.0;
    //     for (i, value1) in c.iter().enumerate().take(2) {
    //         for (j, value2) in value1.iter().enumerate().take(2) {
    //             for (k, _value3) in value2.iter().enumerate().take(2) {
    //                 accum += (i as f64 * u + (1 - i) as f64 * (1.0 - u))
    //                     * (j as f64 * v + (1 - j) as f64 * (1.0 - v))
    //                     * (k as f64 * w + (1 - k) as f64 * (1.0 - w))
    //                     * c[i][j][k];
    //             }
    //         }
    //     }
    //     accum
    // }
    fn perlin_interp(c: &[Vec<Vec<Vec3>>], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for (i, value1) in c.iter().enumerate().take(2) {
            for (j, value2) in value1.iter().enumerate().take(2) {
                for (k, _value3) in value2.iter().enumerate().take(2) {
                    let weight = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                        * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                        * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
                        * c[i][j][k].dot(weight);
                }
            }
        }
        accum
    }

    fn perlin_generate_perm() -> Vec<i32> {
        let mut p = vec![0; POINT_COUNT];
        for (i, value) in p.iter_mut().enumerate().take(POINT_COUNT) {
            *value = i as i32;
        }
        Perlin::permute(&mut p, POINT_COUNT);
        p
    }

    fn permute(p: &mut [i32], n: usize) {
        for i in (1..n).rev() {
            let target = random_i32(0, i as i32);
            p.swap(i, target as usize);
        }
    }
    pub fn turb(&self, p: &Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut tmp_p = p.clone();
        let mut weight = 1.0;
        for _ in 0..depth {
            accum += weight * self.noise(&tmp_p);
            weight *= 0.5;
            tmp_p *= 2.0;
        }
        accum.abs()
    } //depth默认值为7
}
