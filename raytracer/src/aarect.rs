use crate::aabb::AaBb;
use crate::hittable::{Hit, HitRecord};
use crate::material::Material;
use crate::rtweekend::random;
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
pub struct XZRect {
    mp: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}
impl XZRect {
    pub fn new(mp: Arc<dyn Material>, x0: f64, x1: f64, z0: f64, z1: f64, k: f64) -> Self {
        Self {
            mp,
            x0,
            x1,
            z0,
            z1,
            k,
        }
    }
}
impl Hit for XZRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig().y()) / r.dir().y();
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.orig().x() + t * r.dir().x();
        let z = r.orig().z() + t * r.dir().z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        Some(HitRecord::new(
            r.at(t),
            t,
            (x - self.x0) / (self.x1 - self.x0),
            (z - self.z0) / (self.z1 - self.z0),
            &Vec3::new(0.0, 1.0, 0.0),
            r.clone(),
            &*self.mp,
        ))
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AaBb> {
        Some(AaBb::new(
            Point3::new(self.x0, self.k - 0.0001, self.z0),
            Point3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }
    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f64 {
        if let Some(rec) = self.hit(&Ray::new(o.clone(), v.clone(), 0.0), 0.001, f64::INFINITY) {
            let area = (self.x1 - self.x0) * (self.z1 - self.z0);
            let distance_squared = rec.t * rec.t * v.squared_length();
            let cosine = (v.dot(rec.normal) / v.length()).abs();
            distance_squared / (cosine * area)
        } else {
            0.0
        }
    }
    fn random(&self, o: &Vec3) -> Vec3 {
        let random_point = Point3::new(random(self.x0, self.x1), self.k, random(self.z0, self.z1));
        random_point - o.clone()
    }
}
pub struct YZRect {
    mp: Arc<dyn Material>,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}
impl YZRect {
    pub fn new(mp: Arc<dyn Material>, y0: f64, y1: f64, z0: f64, z1: f64, k: f64) -> Self {
        Self {
            mp,
            y0,
            y1,
            z0,
            z1,
            k,
        }
    }
}
impl Hit for YZRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig().x()) / r.dir().x();
        if t < t_min || t > t_max {
            return None;
        }
        let y = r.orig().y() + t * r.dir().y();
        let z = r.orig().z() + t * r.dir().z();
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }
        Some(HitRecord::new(
            r.at(t),
            t,
            (y - self.y0) / (self.y1 - self.y0),
            (z - self.z0) / (self.z1 - self.z0),
            &Vec3::new(1.0, 0.0, 0.0),
            r.clone(),
            &*self.mp,
        ))
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AaBb> {
        Some(AaBb::new(
            Point3::new(self.k - 0.0001, self.y0, self.z0),
            Point3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }
}
