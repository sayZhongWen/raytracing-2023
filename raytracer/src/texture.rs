use crate::perlin::*;
use crate::vec3::{Color, Point3};
use std::sync::Arc;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}
pub struct SolidColor {
    color_value: Color,
}
impl SolidColor {
    pub fn new(c: Color) -> Self {
        Self { color_value: c }
    }
    // pub fn new_rgb(r: f64, g: f64, b: f64) -> Self {
    //     Self {
    //         color_value: Color::new(r, g, b),
    //     }
    // }
}
impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.color_value.clone()
    }
}

pub struct CheckerTexture {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}

impl CheckerTexture {
    //Tip:构造函数和book2上是反的
    pub fn new_color(c1: Color, c2: Color) -> Self {
        Self {
            odd: Arc::new(SolidColor::new(c1)),
            even: Arc::new(SolidColor::new(c2)),
        }
    }
    // pub fn new_arc(odd: Arc<dyn Texture>, even: Arc<dyn Texture>) -> Self {
    //     Self { odd, even }
    // }
}
impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = ((10.0 * p.x()).sin()) * ((10.0 * p.y()).sin()) * ((10.0 * p.z()).sin());
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}
impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}
impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        self.noise.noise(&(self.scale * p.clone())) * Color::ones()
    }
}
