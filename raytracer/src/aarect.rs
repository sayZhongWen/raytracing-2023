use crate::aabb::AaBb;
use crate::hittable::{Hit, HitRecord};
use crate::material::Material;
use crate::vec3::Point3;
use crate::{Ray, Vec3};
use std::sync::Arc;

pub struct XYRect {
    mp: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}
impl XYRect {
    pub fn new(mp: Arc<dyn Material>, x0: f64, x1: f64, y0: f64, y1: f64, k: f64) -> Self {
        Self {
            mp,
            x0,
            x1,
            y0,
            y1,
            k,
        }
    }
}
impl Hit for XYRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig().z()) / r.dir().z();
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.orig().x() + t * r.dir().x();
        let y = r.orig().y() + t * r.dir().y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        Some(HitRecord::new(
            r.at(t),
            t,
            (x - self.x0) / (self.x1 - self.x0),
            (y - self.y0) / (self.y1 - self.y0),
            &Vec3::new(0.0, 0.0, 1.0),
            r.clone(),
            &*self.mp,
        ))
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AaBb> {
        Some(AaBb::new(
            Point3::new(self.x0, self.y0, self.k - 0.0001),
            Point3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}
