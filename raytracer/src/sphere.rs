use crate::{hittable::*, vec3::*, Ray};
use crate::material::Material;

#[derive(Clone)]
pub struct Sphere<M:Material> {
    center: Vec3,
    radius: f64,
    material: M,
}
impl <M:Material>Sphere<M> {
    pub fn new(center: Vec3, radius: f64,material:M) -> Self {
        Self { center, radius ,material}
    }
}
impl <M:Material>Hit for Sphere<M> {
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
        let root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            let root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let p = r.at(root);
        let outward_normal = (p.clone() - self.center.clone()) / self.clone().radius;
        Some(HitRecord::new(p, root, outward_normal, r,&self.material))
    }
}
