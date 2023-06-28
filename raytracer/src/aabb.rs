use crate::{vec3::*, Ray};
use std::mem::swap;
#[derive(Clone)]
pub struct AaBb {
    minimum: Vec3,
    maximum: Vec3,
}
impl AaBb {
    pub fn new(minimum: Vec3, maximum: Vec3) -> Self {
        Self { minimum, maximum }
    }
    pub fn min(&self) -> Vec3 {
        self.minimum.clone()
    }
    pub fn max(&self) -> Vec3 {
        self.maximum.clone()
    }
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        let mut t_min = t_min;
        let mut t_max = t_max;
        for a in 0..3 {
            let inv_d = 1.0 / r.dir()[a]; //1.0f?
            let mut t0 = (self.min()[a] - r.orig()[a]) * inv_d;
            let mut t1 = (self.max()[a] - r.orig()[a]) * inv_d;
            if inv_d < 0.0 {
                swap(&mut t0, &mut t1);
            } //？
            t_min = t0.max(t_min);
            t_max = t1.min(t_max);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}
pub fn surrounding_box(box0: AaBb, box1: AaBb) -> AaBb {
    let small = Vec3::new(
        box0.min().x().min(box1.min().x()),
        box0.min().y().min(box1.min().y()),
        box0.min().z().min(box1.min().z()),
    );
    let big = Vec3::new(
        box0.max().x().max(box1.max().x()),
        box0.max().y().max(box1.max().y()),
        box0.max().z().max(box1.max().z()),
    );
    AaBb::new(small, big)
}
