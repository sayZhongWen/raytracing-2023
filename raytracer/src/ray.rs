use crate::vec3::*;
#[derive(Clone)]
pub struct Ray {
    orig: Vec3,
    dir: Vec3,
}
impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Ray {
        Ray { orig, dir }
    }
    pub fn orig(&self) -> Vec3 {
        self.orig.clone()
    }
    pub fn dir(&self) -> Vec3 {
        self.dir.clone()
    }
    pub fn at(&self, t: f64) -> Vec3 {
        self.orig.clone() + t * self.dir.clone()
    }
}
