use crate::hittable::HitRecord;
use crate::onb::Onb;
use crate::rtweekend::random_f64;
use crate::{ray::*, texture::*, vec3::*};
use std::f64::consts::PI;
use std::sync::Arc;

//有关生命周期的部分学习了https://zhuanlan.zhihu.com/p/441138623
pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, pdf: &mut f64) -> Option<(Ray, Color)> {
        None
    }
    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        0.0
    }
    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::zero()
    }
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, pdf: &mut f64) -> Option<(Ray, Color)> {
        // let mut scatter_direction = rec.normal.clone() + random_unit_vector();
        // if scatter_direction.near_zero() {
        //     scatter_direction = rec.clone().normal;
        // }
        // let dir = scatter_direction.unit_vector();
        let mut uvw = Onb {
            axis: vec![Vec3::zero(), Vec3::zero(), Vec3::zero()],
        };
        uvw.build_from_w(&rec.normal);
        let dir = uvw.local_vector(&random_cosine_direction()).unit_vector();
        // *pdf = rec.normal.dot(dir.clone()) / PI;
        *pdf = uvw.w().dot(dir.clone()) / PI;
        Some((
            Ray::new(rec.p.clone(), dir, r_in.time()),
            self.albedo.value(rec.u, rec.v, &rec.p),
        ))
    }
    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = rec.normal.dot(scattered.dir().unit_vector());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, pdf: &mut f64) -> Option<(Ray, Vec3)> {
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, pdf: &mut f64) -> Option<(Ray, Vec3)> {
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

pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}
impl DiffuseLight {
    // pub fn new_arc(emit: Arc<dyn Texture>) -> Self {
    //     Self { emit }
    // }
    pub fn new_color(c: Color) -> Self {
        Self {
            emit: Arc::new(SolidColor::new(c)),
        }
    }
}
impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord, _pdf: &mut f64) -> Option<(Ray, Vec3)> {
        None
    }
    fn emitted(&self, _r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        if rec.front_face {
            self.emit.value(u, v, p)
        } else {
            Color::zero()
        }
    }
}
pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new_color(c: Color) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(c)),
        }
    }
    // pub fn new_text(a: Arc<dyn Texture>) -> Self {
    //     Self { albedo: a }
    // }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, pdf: &mut f64) -> Option<(Ray, Vec3)> {
        Some((
            Ray::new(rec.p.clone(), random_in_unit_sphere(), r_in.time()),
            self.albedo.value(rec.u, rec.v, &rec.p),
        ))
    }
}
