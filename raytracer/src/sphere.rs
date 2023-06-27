use crate::material::Material;
use crate::{hittable::*, vec3::*, Ray};

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
impl<M: Material> Hit for Sphere<M> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
        Some(HitRecord::new(p, root, &outward_normal, r, &self.material))
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
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
        Some(HitRecord::new(p, root, &outward_normal, r, &self.material))
    }
}
