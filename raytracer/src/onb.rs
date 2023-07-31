use crate::Vec3;
use std::ops::Index;

pub struct Onb {
    pub axis: Vec<Vec3>,
}
impl Onb {
    pub fn u(&self) -> Vec3 {
        self.axis[0].clone()
    }
    pub fn v(&self) -> Vec3 {
        self.axis[1].clone()
    }
    pub fn w(&self) -> Vec3 {
        self.axis[2].clone()
    }
    pub fn local(&self, a: f64, b: f64, c: f64) -> Vec3 {
        a * self.u() + b * self.v() + c * self.w()
    }
    pub fn local_vector(&self, a: &Vec3) -> Vec3 {
        a.x() * self.u() + a.y() * self.v() + a.z() * self.w()
    }
    pub fn build_from_w(&mut self, n: &Vec3) {
        self.axis[2] = n.unit_vector();
        let a = if self.w().x().abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        self.axis[1] = self.w().cross(a).unit_vector();
        self.axis[0] = self.w().cross(self.v());
    }
}
impl Index<usize> for Onb {
    type Output = Vec3;
    fn index(&self, index: usize) -> &Self::Output {
        &self.axis[index]
    }
}
