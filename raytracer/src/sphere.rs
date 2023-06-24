use crate::{hittable::*, vec3::*, Ray};
#[derive(Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
}
impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}
impl Hit for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig() - self.clone().center;
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
        let outward_normal = (p.clone() - self.clone().center) / self.clone().radius;
        Some(HitRecord::new(p, root, outward_normal, r))
    }
}
