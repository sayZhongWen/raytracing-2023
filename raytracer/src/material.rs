use crate::{ray::*, rtweekend::*, vec3::*};
use crate::hittable::HitRecord;
//有关生命周期的部分学习了https://zhuanlan.zhihu.com/p/441138623
pub trait Material{
    fn scatter(&self,r_in:&Ray,rec: HitRecord)->Option<(Ray,Vec3)>;
}
pub struct Lambertian{
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: &Vec3)->Self{Self{albedo:Vec3::new(albedo.x(),albedo.y(),albedo.z())}}
}
impl Material for Lambertian{
    fn scatter(&self, r_in: &Ray, rec: HitRecord) -> Option<(Ray, Vec3)> {
        let mut scatter_direction=rec.normal.clone()+random_unit_vector();
        if scatter_direction.near_zero(){
            scatter_direction=rec.normal;
        }
        Some((Ray::new(rec.p,scatter_direction),self.albedo.clone()))
    }
}



pub struct Metal{
    pub albedo:Vec3,
    pub fuzz:f64,
}
impl Metal{
    pub fn new(albedo: &Vec3,f:f64)->Self{
        Self{
        albedo:Vec3::new(albedo.x(),albedo.y(),albedo.z()),
        fuzz:if f<1.0 {f}else{1.0},
        }}
}
impl Material  for Metal{
    fn scatter(&self,r_in: &Ray, rec: HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(r_in.dir().unit_vector(),rec.normal.clone());
        let scattered=Ray::new(rec.p,reflected+self.fuzz*random_in_unit_sphere());
        let attenuation=self.albedo.clone();
        if scattered.dir().dot(rec.normal)>0.0 {
            Some((scattered,attenuation))
        }
        else{
            None
        }
    }
}