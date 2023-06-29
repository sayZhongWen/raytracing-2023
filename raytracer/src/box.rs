use crate::aabb::AaBb;
use crate::hittable::{Hit, HitRecord};
use crate::material::Material;
use crate::vec3::Point3;
use crate::{aarect::*, hittable_list::*, Ray};
use std::sync::Arc;

pub struct Bbox {
    box_min: Point3,
    box_max: Point3,
    sides: HittableList,
}

impl Bbox {
    pub fn new(p0: &Point3, p1: &Point3, material: Arc<dyn Material>) -> Self {
        let mut sides = HittableList::new();
        sides.add(Arc::new(XYRect::new(
            material.clone(),
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
        )));
        sides.add(Arc::new(XYRect::new(
            material.clone(),
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
        )));
        sides.add(Arc::new(XZRect::new(
            material.clone(),
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p1.y(),
        )));
        sides.add(Arc::new(XZRect::new(
            material.clone(),
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p0.y(),
        )));
        sides.add(Arc::new(YZRect::new(
            material.clone(),
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p1.x(),
        )));
        sides.add(Arc::new(YZRect::new(
            material,
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p0.x(),
        )));
        Self {
            box_min: p0.clone(),
            box_max: p1.clone(),
            sides,
        }
    }
}

impl Hit for Bbox {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AaBb> {
        Some(AaBb::new(self.box_min.clone(), self.box_max.clone()))
    }
}
