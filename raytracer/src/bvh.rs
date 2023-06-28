use crate::aabb::{surrounding_box, AaBb};
use crate::hittable::{Hit, HitRecord};
use crate::hittable_list::HittableList;
use crate::rtweekend::random_i32;
use crate::Ray;
use std::sync::Arc;

pub struct BvhNode {
    left: Arc<dyn Hit>,
    right: Arc<dyn Hit>,
    bbox: AaBb,
}

impl BvhNode {
    pub fn newnew(list: HittableList, time0: f64, time1: f64) -> Arc<dyn Hit> {
        BvhNode::split(list.objects, time0, time1)
    }
    fn split(src_objects: Vec<Arc<dyn Hit>>, time0: f64, time1: f64) -> Arc<dyn Hit> {
        let mut objects = src_objects; //
        let axis: i32 = random_i32(0, 2);
        objects.sort_by(|a, b| {
            a.bounding_box(time0, time1).unwrap().min()[axis]
                .partial_cmp(&b.bounding_box(time0, time1).unwrap().min()[axis])
                .unwrap()
        });
        let mut left = objects[0].clone(); //
        let mut right = objects[0].clone();
        match objects.len() {
            1 => {}
            2 => right = objects[1].clone(),
            // 2 => {
            //     if objects[0].bounding_box(time0, time1).unwrap().min()[axis]
            //         < objects[1].bounding_box(time0, time1).unwrap().min()[axis]
            //     {
            //         right = objects[1].clone();
            //     } else {
            //         left = objects[1].clone();
            //     }
            // }
            _ => {
                let mut l = objects;
                let r = l.split_off(l.len() / 2);
                left = BvhNode::split(l, time0, time1);
                right = BvhNode::split(r, time0, time1);
            }
        };
        let bbox = surrounding_box(
            left.bounding_box(time0, time1).unwrap(),
            right.bounding_box(time0, time1).unwrap(),
        );
        Arc::new(Self { left, right, bbox })
    }
}
impl Hit for BvhNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bbox.hit(r, t_min, t_max) {
            return None;
        }
        let hit_left = self.left.hit(r, t_min, t_max);
        let hit_right = self.right.hit(r, t_min, t_max);
        match (hit_left, hit_right) {
            (Some(hit_left), Some(hit_right)) => {
                if hit_left.t < hit_right.t {
                    Some(hit_left)
                } else {
                    Some(hit_right)
                }
            }
            (Some(hit_left), None) => Some(hit_left),
            (None, Some(hit_right)) => Some(hit_right),
            (None, None) => None,
        }
        // if let Some(rec) = self.left.hit(r, t_min, t_max) {
        //     if let Some(rec) = self.right.hit(r, t_min, rec.t) {
        //         return Some(rec);
        //     } else if let Some(rec) = self.right.hit(r, t_min, t_max) {
        //         return Some(rec);
        //     }
        // }
        // None
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AaBb> {
        Some(self.bbox.clone())
    }
}
