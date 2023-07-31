use crate::hittable::Hit;
use crate::onb::Onb;
use crate::rtweekend::random_f64;
use crate::vec3::{random_cosine_direction, Point3};
use crate::Vec3;
use std::f64::consts::PI;
use std::sync::Arc;

pub trait Pdf: Send + Sync {
    fn value(&self, direction: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}
pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    pub fn new(w: &Vec3) -> Self {
        let mut uvw = Onb {
            axis: vec![Vec3::zero(), Vec3::zero(), Vec3::zero()],
        };
        uvw.build_from_w(w);
        Self { uvw }
    }
}
impl Pdf for CosinePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine = direction.unit_vector().dot(self.uvw.w());
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
    fn generate(&self) -> Vec3 {
        self.uvw.local_vector(&random_cosine_direction())
    }
}

pub struct HittablePdf {
    o: Point3,
    ptr: Arc<dyn Hit>,
}

impl HittablePdf {
    pub fn new(ptr: Arc<dyn Hit>, o: Point3) -> Self {
        Self { ptr, o }
    }
}
impl Pdf for HittablePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        self.ptr.pdf_value(&self.o, direction)
    }
    fn generate(&self) -> Vec3 {
        self.ptr.random(&self.o)
    }
}

pub struct MixturePdf {
    p: Vec<Arc<dyn Pdf>>,
}

impl MixturePdf {
    pub fn new(p0: Arc<dyn Pdf>, p1: Arc<dyn Pdf>) -> Self {
        let p = vec![p0, p1];
        Self { p }
    }
}

impl Pdf for MixturePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }
    fn generate(&self) -> Vec3 {
        if random_f64() < 0.5 {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        }
    }
}
