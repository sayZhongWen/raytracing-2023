use crate::aabb::{surrounding_box, AaBb};
use crate::{hittable::*, ray::*};
use std::sync::Arc;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hit>>,
}
impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn add(&mut self, object: Arc<dyn Hit>) {
        self.objects.push(object)
    }
    // pub fn clear(&mut self) {
    //     self.objects.clear()
    // }
}
impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}
impl Hit for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut tmp_rec: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for object in self.objects.iter() {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                tmp_rec = Some(rec);
            }
        }
        tmp_rec
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AaBb> {
        if self.objects.is_empty() {
            return None;
        }
        if let Some(mut temp_box) = self.objects[0].bounding_box(time0, time1) {
            let mut output_box = temp_box;
            for a in 1..self.objects.len() {
                match self.objects[a].bounding_box(time0, time1) {
                    None => {
                        return None;
                    }
                    other => temp_box = other.unwrap(),
                };
                output_box = surrounding_box(output_box, temp_box);
            }
            Some(output_box)
        } else {
            None
        }
    } //自己改写的方法
}
