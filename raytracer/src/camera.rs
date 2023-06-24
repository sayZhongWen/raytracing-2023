use crate::{vec3::*,ray::*};
pub struct Camera{
    origin:Vec3,
    lower_left_corner:Vec3,
    horizontal:Vec3,
    vertical:Vec3,
}
impl Camera{
    pub fn new()->Self{
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
        let ori=Vec3::new(0.0, 0.0, 0.0);
        let hori=Vec3::new(viewport_width, 0.0, 0.0);
        let vert=Vec3::new(0.0, viewport_height, 0.0);
        let llc=ori.clone()-hori.clone()/2.0-vert.clone()/2.0-Vec3::new(0.0, 0.0, focal_length);
        Self{
            origin : ori,
            horizontal : hori,
            vertical : vert,
            lower_left_corner : llc,
        }
    }
    pub fn get_ray(&self,u:f64,v:f64)->Ray{
        Ray::new(self.origin.clone(),self.lower_left_corner.clone() + self.horizontal.clone() * u + self.vertical.clone() * v
            - self.origin.clone())
    }
}