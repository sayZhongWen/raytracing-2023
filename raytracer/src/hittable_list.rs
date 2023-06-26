use crate::{hittable::*, ray::*};
pub struct HittableList {
    pub objects: Vec<Box<dyn Hit>>,
}
impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn add(&mut self, object: Box<dyn Hit>) {
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
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut tmp_rec: Option<HitRecord> = None;
        let mut closes_so_far = t_max;
        for object in self.objects.iter() {
            if let Some(rec) = object.hit(r.clone(), t_min, closes_so_far) {
                closes_so_far = rec.t;
                tmp_rec = Some(rec);
            }
        }
        tmp_rec
    }
}
