use crate::{ray::*, vec3::*};
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    // u:Vec3,
    // v:Vec3,
    // w:Vec3,
    // lens_radius:f64,
}
impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
        let ori = Vec3::new(0.0, 0.0, 0.0);
        let hori = Vec3::new(viewport_width, 0.0, 0.0);
        let vert = Vec3::new(0.0, viewport_height, 0.0);
        let llc = ori.clone()
            - hori.clone() / 2.0
            - vert.clone() / 2.0
            - Vec3::new(0.0, 0.0, focal_length);
        Self {
            origin: ori,
            horizontal: hori,
            vertical: vert,
            lower_left_corner: llc,
        }
    }
    // pub fn new(vfov:f64,aspect_ratio:f64)->Self{
    //     let theta=degrees_to_radians(vfov);
    //     let h=tan(theta/2);
    //     let viewpoint_height=2.0*h;
    //     let viewpoint_width=aspect_ratio*viewpoint_height;
    //     let focal_length = 1.0;
    //     let ori = Vec3::new(0.0, 0.0, 0.0);
    //     let hori = Vec3::new(viewport_width, 0.0, 0.0);
    //     let vert = Vec3::new(0.0, viewport_height, 0.0);
    //     let llc = ori.clone()
    //         - hori.clone() / 2.0
    //         - vert.clone() / 2.0
    //         - Vec3::new(0.0, 0.0, focal_length);
    //     Self {
    //         origin: ori,
    //         horizontal: hori,
    //         vertical: vert,
    //         lower_left_corner: llc,
    //     }
    // }

    // pub fn new(lookfrom:Vec3,lookat:Vec3,vup:Vec3,vfov:f64,aspect_ratio:f64)->Self{
    //     let theta=degrees_to_radians(vfov);
    //     let h=tan(theta/2);
    //     let viewpoint_height=2.0*h;
    //     let viewpoint_width=aspect_ratio*viewpoint_height;
    //     let w = (lookfrom.clone() - lookat).unit_vector();
    //     let u = (vup.cross( w.clone())).unit_vector();
    //     let v = w.cross( u.clone());
    //     let ori = lookfrom;
    //     let hori = viewpoint_width*u;
    //     let vert = viewpoint_height*v;
    //     let llc = ori.clone()
    //         - hori.clone() / 2.0
    //         - vert.clone() / 2.0
    //         - w;
    //     Self {
    //         origin: ori,
    //         horizontal: hori,
    //         vertical: vert,
    //         lower_left_corner: llc,
    //     }
    // }

    // pub fn new(lookfrom:Vec3,lookat:Vec3,vup:Vec3,vfov:f64,aspect_ratio:f64,aperture:f64,focus_dist:f64)->Self{
    //     let theta=degrees_to_radians(vfov);
    //     let h=tan(theta/2);
    //     let viewpoint_height=2.0*h;
    //     let viewpoint_width=aspect_ratio*viewpoint_height;
    //     let w = (lookfrom.clone() - lookat).unit_vector();
    //     let u = (vup.cross( w.clone())).unit_vector();
    //     let v = w.cross( u.clone());
    //     let ori = lookfrom;
    //     let hori = focus_dist*viewpoint_width*u;
    //     let vert = focus_dist*viewpoint_height*v;
    //     let llc = ori.clone()
    //         - hori.clone() / 2.0
    //         - vert.clone() / 2.0
    //         - focus_dist*w;
    //     Self {
    //         origin: ori,
    //         horizontal: hori,
    //         vertical: vert,
    //         lower_left_corner: llc,
    //         u,
    //         v,
    //         w,
    //         lens_radius:aperture/2,
    //     }
    // }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin.clone(),
            self.lower_left_corner.clone()
                + self.horizontal.clone() * u
                + self.vertical.clone() * v
                - self.origin.clone(),
        )
    }
}
