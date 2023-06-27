use crate::vec3::*;
#[derive(Clone)]
pub struct Ray {
    orig: Vec3,
    dir: Vec3,
    tm: f64,
}
impl Ray {
    pub fn new(orig: Vec3, dir: Vec3, tm: f64) -> Ray {
        Ray { orig, dir, tm }
    }
    pub fn orig(&self) -> Vec3 {
        self.orig.clone()
    }
    pub fn dir(&self) -> Vec3 {
        self.dir.clone()
    }
    pub fn time(&self) -> f64 {
        self.tm
    }
    pub fn at(&self, t: f64) -> Vec3 {
        self.orig.clone() + t * self.dir.clone()
    }
}
