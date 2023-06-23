use crate::vec3::*;
pub struct Ray{
    orig: Vec3,
    dir: Vec3,
}
impl Ray{
    pub fn new(orig: Vec3,dir: Vec3) -> Ray{
        Ray {
            orig,
            dir,
        }
    }
    pub fn orig(&self) -> Vec3 {
        let orig1=self.orig.clone();
        orig1
    }
    pub fn dir(&self) -> Vec3 {
        let dir1=self.dir.clone();
        dir1
    }
    pub fn at(&self, t: f64) -> Vec3 {
        let orig1=self.orig.clone();
        let dir1=self.dir.clone();
        let v=(orig1+t*dir1).clone();
        v
    }
}