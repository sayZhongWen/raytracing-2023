use crate::aabb::AaBb;
use crate::hittable::{Hit, HitRecord};
use crate::material::{Isotropic, Material};
use crate::rtweekend::random_f64;
// use crate::texture::Texture;
use crate::vec3::Color;
use crate::{Ray, Vec3};
use std::sync::Arc;

pub struct ConstantMedium {
    pub boundary: Arc<dyn Hit>,
    pub phase_function: Arc<dyn Material>,
    neg_inv_density: f64,
}
impl ConstantMedium {
    // pub fn new_text(b: Arc<dyn Hit>, d: f64, a: Arc<dyn Texture>) -> Self {
    //     Self {
    //         boundary: b,
    //         phase_function: Arc::new(Isotropic::new_text(a)),
    //         neg_inv_density: -1.0 / d,
    //     }
    // }
    pub fn new_color(b: Arc<dyn Hit>, d: f64, c: Color) -> Self {
        Self {
            boundary: b,
            phase_function: Arc::new(Isotropic::new_color(c)),
            neg_inv_density: -1.0 / d,
        }
    }
}

impl Hit for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // let enable_degbug = false;
        // let debugging= enable_degbug && random_f64() < 0.00001;
        if let Some(mut rec1) = self.boundary.hit(r, -f64::INFINITY, f64::INFINITY) {
            if let Some(mut rec2) = self.boundary.hit(r, rec1.t + 0.0001, f64::INFINITY) {
                // if DEBUGGING
                if rec1.t < t_min {
                    rec1.t = t_min;
                }
                if rec2.t > t_max {
                    rec2.t = t_max;
                }
                if rec1.t >= rec2.t {
                    return None;
                }
                if rec1.t < 0.0 {
                    rec1.t = 0.0;
                }
                let ray_length = r.dir().length();
                let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                let hit_distance = self.neg_inv_density * (random_f64().ln());
                if hit_distance > distance_inside_boundary {
                    return None;
                }
                let tt = rec1.t + hit_distance / ray_length;
                return Some(HitRecord::new(
                    r.at(tt),
                    tt,
                    0.0,
                    0.0,
                    &Vec3::new(1.0, 0.0, 0.0),
                    r.clone(),
                    &*self.phase_function,
                ));
            } else {
                None
            }
        } else {
            None
        }
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AaBb> {
        self.boundary.bounding_box(time0, time1)
    }
}
