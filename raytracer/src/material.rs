use crate::hittable::HitRecord;
use crate::rtweekend::random_f64;
use crate::{ray::*, texture::*, vec3::*};
use std::sync::Arc;

//有关生命周期的部分学习了https://zhuanlan.zhihu.com/p/441138623
pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)>;
}
pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new_color(a: Color) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(a)),
        }
    }
    pub fn new_arc(a: Arc<dyn Texture>) -> Self {
        Self { albedo: a }
    }
}
impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal.clone() + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.clone().normal;
        }
        Some((
            Ray::new(rec.p.clone(), scatter_direction, r_in.time()),
            self.albedo.value(rec.u, rec.v, &rec.p),
        ))
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
        let scattered = Ray::new(
            rec.p.clone(),
            reflected + self.fuzz * random_in_unit_sphere(),
            r_in.time(),
        );
        let attenuation = self.albedo.clone();
        if scattered.dir().dot(rec.normal.clone()) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
#[derive(Clone)]
pub struct Dielectric {
    pub ir: f64,
}
impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }
    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}
impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let attenuation = Vec3::ones();
        let refraction_ratio: f64 = if rec.clone().front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = r_in.dir().unit_vector();
        let cos_theta: f64 = (-unit_direction.clone()).dot(rec.normal.clone()).min(1.0);
        let sin_theta: f64 = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > random_f64()
        {
            reflect(&unit_direction, &rec.normal)
        } else {
            refract(&unit_direction, &rec.normal, refraction_ratio)
        };
        Some((Ray::new(rec.p.clone(), direction, r_in.time()), attenuation))
    }
}
