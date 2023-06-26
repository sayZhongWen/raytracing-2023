use crate::hittable::HitRecord;
use std::num;
use crate::{ray::*, vec3::*};
//有关生命周期的部分学习了https://zhuanlan.zhihu.com/p/441138623
pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)>;
}
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: &Vec3) -> Self {
        Self {
            albedo: Vec3::new(albedo.x(), albedo.y(), albedo.z()),
        }
    }
}
impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let mut scatter_direction = rec.normal.clone() + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }
        Some((Ray::new(rec.p.clone(), scatter_direction), self.albedo.clone()))
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}
impl Metal {
    pub fn new(albedo: &Vec3, f: f64) -> Self {
        Self {
            albedo: Vec3::new(albedo.x(), albedo.y(), albedo.z()),
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
}
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(&r_in.dir().unit_vector(), &rec.normal);
        let scattered = Ray::new(rec.p.clone(), reflected + self.fuzz * random_in_unit_sphere());
        let attenuation = self.albedo.clone();
        if scattered.dir().dot(rec.normal.clone()) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub ir: f64,
}
impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }
    // pub fn reflectance(cosine:f64,ref_idx:f64)->f64{
    //     let r0 = (1.0-ref_idx) / (1.0+ref_idx);
    //     r0 = r0*r0;
    //     r0 + (1-r0)*(1.0 - cosine).pow(5)
    // }
}
impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let attenuation = Vec3::ones();
        let refraction_ratio: f64 = if rec.front_face.clone() {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = r_in.dir().unit_vector();
        // let cos_theta:f64=(-unit_direction.clone()).dot(rec.normal.clone()).min(1.0);
        // let sin_theta:f64=(1.0 - cos_theta*cos_theta).sqrt();
        // let cannot_reflect:bool=refraction_ratio*sin_theta>1.0;
        // let cannot_reflect||
        // let direction=if cannot_reflect{
        //     reflect(unit_direction,rec.normal)
        // }else{
        //     refract(unit_direction,rec.normal,refraction_ratio)
        // };
        // Some((Ray::new(rec.p, direction), attenuation))
        let refracted=refract(&unit_direction,&rec.normal,refraction_ratio);
        Some((Ray::new(rec.p.clone(), refracted), attenuation))
    }
}