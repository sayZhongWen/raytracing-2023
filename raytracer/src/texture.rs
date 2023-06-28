use crate::perlin::*;
use crate::vec3::{Color, Point3};
// extern crate stb_image;
// use stb_image::image::*;
use crate::rtweekend::clamp;
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
        0.5 * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turb(p, 7)).sin()) * Color::ones()
    }
}

pub struct ImageTexture {
    data: Vec<u8>,
    width: usize,
    height: usize,
    bytes_per_scanline: usize,
}
impl ImageTexture {
    const BYTES_PER_PIXEL: usize = 3;
    pub fn new(data: Vec<u8>, width: usize, height: usize) -> Self {
        Self {
            data,
            width,
            height,
            bytes_per_scanline: ImageTexture::BYTES_PER_PIXEL * width,
        }
    }
}

impl Texture for ImageTexture {
    //有关读取图片和rgb的data的内容是参考了https://github.com/fralken/ray-tracing-the-next-week/blob/master/src/main.rs的方法
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        if self.data.is_empty() {
            return Color::new(0.0, 1.0, 1.0);
        }
        let uu = clamp(u, 0.0, 1.0);
        let vv = 1.0 - clamp(v, 0.0, 1.0);
        let mut i = (uu * self.width as f64) as usize;
        let mut j = (vv * self.height as f64) as usize;
        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }
        const COLOR_SCALE: f64 = 1.0 / 255.0;
        let idx = ImageTexture::BYTES_PER_PIXEL * i + self.bytes_per_scanline * j;
        let r = self.data[idx] as f64 * COLOR_SCALE;
        let g = self.data[idx + 1] as f64 * COLOR_SCALE;
        let b = self.data[idx + 2] as f64 * COLOR_SCALE;
        Color::new(r, g, b)
    }
}
