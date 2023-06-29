use crate::aabb::AaBb;
use crate::rtweekend::degrees_to_radians;
use crate::{material::*, ray::*, vec3::*};
use std::sync::Arc;

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
pub struct Translate {
    ptr: Arc<dyn Hit>,
    offset: Vec3,
}

impl Translate {
    pub fn new(ptr: Arc<dyn Hit>, offset: Vec3) -> Self {
        Self { ptr, offset }
    }
}

impl Hit for Translate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(r.orig() - self.offset.clone(), r.dir(), r.time());
        if let Some(rec) = self.ptr.hit(&moved_r, t_min, t_max) {
            return Some(HitRecord::new(
                rec.p + self.offset.clone(),
                rec.t,
                rec.u,
                rec.v,
                &rec.normal,
                moved_r,
                rec.material,
            ));
        }
        None
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AaBb> {
        if let Some(output_box) = self.ptr.bounding_box(time0, time1) {
            return Some(AaBb::new(
                output_box.min() + self.offset.clone(),
                output_box.max() + self.offset.clone(),
            ));
        }
        None
    }
}
pub struct RotateY {
    ptr: Arc<dyn Hit>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<AaBb>,
}

impl RotateY {
    pub fn new(ptr: Arc<dyn Hit>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bbox = ptr.bounding_box(0.0, 1.0); //
        match bbox {
            None => {
                bbox = None;
            }
            Some(other) => {
                let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
                let mut max = Point3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);
                for i in 0..2 {
                    for j in 0..2 {
                        for k in 0..2 {
                            let x = i as f64 * other.max().x() + (1.0 - i as f64) * other.min().x();
                            let y = j as f64 * other.max().y() + (1.0 - j as f64) * other.min().y();
                            let z = k as f64 * other.max().z() + (1.0 - k as f64) * other.min().z();
                            let newx = cos_theta * x + sin_theta * z;
                            let newz = -sin_theta * x + cos_theta * z;
                            let tester = Vec3::new(newx, y, newz);
                            for c in 0..3 {
                                min[c] = min[c].min(tester[c]);
                                max[c] = max[c].max(tester[c]);
                            }
                        }
                    }
                }
                bbox = Some(AaBb::new(min, max));
            }
        }

        Self {
            ptr,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hit for RotateY {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = r.orig();
        let mut direction = r.dir();
        origin[0] = self.cos_theta * r.orig()[0] - self.sin_theta * r.orig()[2];
        origin[2] = self.sin_theta * r.orig()[0] + self.cos_theta * r.orig()[2];
        direction[0] = self.cos_theta * r.dir()[0] - self.sin_theta * r.dir()[2];
        direction[2] = self.sin_theta * r.dir()[0] + self.cos_theta * r.dir()[2];
        let rotated_r = Ray::new(origin, direction, r.time());
        if let Some(rec) = self.ptr.hit(&rotated_r, t_min, t_max) {
            let mut p = rec.p.clone();
            let mut normal = rec.normal.clone();
            p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
            p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];
            normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
            normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];
            return Some(HitRecord::new(
                p,
                rec.t,
                rec.u,
                rec.v,
                &normal,
                rotated_r,
                rec.material,
            ));
        }
        None
    }
    fn bounding_box(&self, _: f64, _: f64) -> Option<AaBb> {
        self.bbox.clone()
    }
}
