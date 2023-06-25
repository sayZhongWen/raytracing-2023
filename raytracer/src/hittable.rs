use crate::{ray::*, vec3::*};
pub trait Hit {
    //此处返回Option<HitRecord>的思想改编自助教分享的https://zhuanlan.zhihu.com/p/436876484
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}
impl HitRecord {
    pub fn new(p: Vec3, t: f64, outward_normal: Vec3, r: Ray) -> Self {
        let front_face = r.dir().dot(outward_normal.clone()) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            p,
            normal,
            t,
            front_face,
        }
    }
}
