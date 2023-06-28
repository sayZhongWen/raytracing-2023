use crate::aabb::AaBb;
use crate::{material::*, ray::*, vec3::*};

pub trait Hit {
    //此处返回Option<HitRecord>的思想改编自助教分享的https://zhuanlan.zhihu.com/p/436876484
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AaBb>;
}
#[derive(Clone)]
pub struct HitRecord<'a> {
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
    pub u: f64,
    pub v: f64,
    pub t: f64,
    pub front_face: bool,
}
impl<'a> HitRecord<'a> {
    pub fn new(
        p: Vec3,
        t: f64,
        u: f64,
        v: f64,
        outward_normal: &Vec3,
        r: Ray,
        material: &'a dyn Material,
    ) -> Self {
        let front_face = r.dir().dot(outward_normal.clone()) < 0.0;
        let normal = if front_face {
            outward_normal.clone()
        } else {
            -outward_normal.clone()
        };
        Self {
            p,
            normal,
            t,
            u,
            v,
            front_face,
            material,
        }
    }
}
