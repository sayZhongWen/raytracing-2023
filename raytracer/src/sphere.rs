use crate::aabb::*;
use crate::material::Material;
use crate::{hittable::*, vec3::*, Ray};
// use std::f64::consts::PI;

#[derive(Clone)]
pub struct Sphere<M: Material> {
    center: Vec3,
    radius: f64,
    material: M,
}
impl<M: Material> Sphere<M> {
    pub fn new(center: Vec3, radius: f64, material: M) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}
// pub fn get_sphere_uv(p: &Point3) -> (f64, f64) {
//     let theta = (-p.y()).acos();
//     let phi = (-p.z()).atan2(p.x()) + PI;
//     (phi / (2.0 * PI), theta / PI)
// }
impl<M: Material> Hit for Sphere<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig() - self.center.clone();
        let a = r.dir().squared_length();
        let half_b = oc.dot(r.dir());
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let p = r.at(root);
        let outward_normal = (p.clone() - self.center.clone()) / self.radius;
        // let (u, v) = get_sphere_uv(&outward_normal);
        Some(HitRecord::new(
            p,
            root,
            // u,
            // v,
            &outward_normal,
            r.clone(),
            &self.material,
        ))
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AaBb> {
        Some(AaBb::new(
            self.center.clone() - Vec3::new(self.radius, self.radius, self.radius),
            self.center.clone() + Vec3::new(self.radius, self.radius, self.radius),
        ))
    }
}

pub struct MovingSphere<M: Material> {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub material: M,
}
impl<M: Material> MovingSphere<M> {
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: M,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }
    pub fn center(&self, time: f64) -> Vec3 {
        self.center0.clone()
            + ((time - self.time0) / (self.time1 - self.time0))
                * (self.center1.clone() - self.center0.clone())
    }
}
impl<M: Material> Hit for MovingSphere<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig() - self.center(r.time());
        let a = r.dir().squared_length();
        let half_b = oc.dot(r.dir());
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let p = r.at(root);
        let outward_normal = (p.clone() - self.center(r.time())) / self.radius;
        Some(HitRecord::new(
            p,
            root,
            &outward_normal,
            r.clone(),
            &self.material,
        ))
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AaBb> {
        let box0 = AaBb::new(
            self.center(_time0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(_time0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1 = AaBb::new(
            self.center(_time1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(_time1) + Vec3::new(self.radius, self.radius, self.radius),
        );
        Some(surrounding_box(box0, box1))
    }
}
