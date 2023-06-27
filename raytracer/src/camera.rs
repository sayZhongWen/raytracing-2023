use crate::rtweekend::random;
use crate::{ray::*, rtweekend::degrees_to_radians, vec3::*};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    // w: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}
impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time0: f64,
        time1: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewpoint_height = 2.0 * h;
        let viewpoint_width = aspect_ratio * viewpoint_height;
        let ww = (lookfrom.clone() - lookat).unit_vector();
        let uu = (vup.cross(ww.clone())).unit_vector();
        let vv = ww.cross(uu.clone());
        let ori = lookfrom;
        let hori = focus_dist * viewpoint_width * uu.clone();
        let vert = focus_dist * viewpoint_height * vv.clone();
        let llc = ori.clone() - hori.clone() / 2.0 - vert.clone() / 2.0 - focus_dist * ww;
        Self {
            origin: ori,
            horizontal: hori,
            vertical: vert,
            lower_left_corner: llc,
            u: uu,
            v: vv,
            // w: ww,
            lens_radius: aperture / 2.0,
            time0,
            time1,
        }
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u.clone() * rd.x() + self.v.clone() * rd.y();
        Ray::new(
            self.origin.clone() + offset.clone(),
            self.lower_left_corner.clone()
                + self.horizontal.clone() * s
                + self.vertical.clone() * t
                - self.origin.clone()
                - offset,
            random(self.time0, self.time1),
        )
    }
}
