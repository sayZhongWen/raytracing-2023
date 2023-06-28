use crate::rtweekend::*;
use crate::vec3::Point3;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ranfloat: Vec<f64>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    pub fn new() -> Self {
        let mut ranfloat = Vec::new();
        for _ in 0..POINT_COUNT {
            ranfloat.push(random_f64())
        }

        Self {
            ranfloat,
            perm_x: Perlin::perlin_generate_perm(),
            perm_y: Perlin::perlin_generate_perm(),
            perm_z: Perlin::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let i = (((4.0 * p.x()) as i32) & 255) as usize;
        let j = (((4.0 * p.y()) as i32) & 255) as usize;
        let k = (((4.0 * p.z()) as i32) & 255) as usize;

        self.ranfloat[(self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize]
    }

    fn perlin_generate_perm() -> Vec<i32> {
        let mut p = vec![0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            p[i] = i as i32;
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
}
