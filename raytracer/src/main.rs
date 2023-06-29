mod aabb;
mod aarect;
mod bvh;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod perlin;
mod ray;
mod rtweekend;
mod sphere;
mod texture;
mod vec3;

use crate::bvh::BvhNode;
use crate::camera::Camera;
use crate::hittable::Hit;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal};
pub use crate::ray::Ray;
use crate::rtweekend::*;
use crate::sphere::{MovingSphere, Sphere};
use color::write_color;

use crate::aarect::XYRect;
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use crate::vec3::{Color, Point3};
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::fs::File;
use std::sync::Arc;
pub use vec3::Vec3;

const AUTHOR: &str = "Dizzy_D";

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn ray_color(r: Ray, background: &Color, world: &dyn Hit, depth: i32) -> Color {
    if depth <= 0 {
        return Vec3::zero();
    }
    return if let Some(rec) = world.hit(&r, 0.001, f64::INFINITY) {
        let emitted = rec.material.emitted(rec.u, rec.v, &rec.p);
        if let Some((scattered, attenuation)) = rec.material.scatter(&r, &rec) {
            emitted + attenuation * ray_color(scattered, background, world, depth - 1)
        } else {
            emitted
        }
    } else {
        background.clone()
    };
}

pub fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let checker = Arc::new(CheckerTexture::new_color(
        Color::new(0.9, 0.9, 0.9),
        Color::new(0.2, 0.3, 0.1),
    ));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new_arc(checker),
    )));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Vec3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );
            if (center.clone() - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::random_f64() * Vec3::random_f64();
                    let sphere_material = Lambertian::new_color(albedo);
                    let center2 = center.clone() + Vec3::new(0.0, random(0.0, 0.5), 0.0);
                    world.add(Arc::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random(0.5, 1.0);
                    let fuzz = random(0.0, 0.5);
                    let sphere_material = Metal::new(&albedo, fuzz);
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Dielectric::new(1.5);
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }
    let material1 = Dielectric::new(1.5);
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = Lambertian::new_color(Color::new(0.4, 0.2, 0.1));
    world.add(Arc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Metal::new(&Vec3::new(0.7, 0.6, 0.5), 0.0);
    world.add(Arc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
    world
}

fn two_spheres() -> HittableList {
    let checker = Arc::new(CheckerTexture::new_color(
        Color::new(0.9, 0.9, 0.9),
        Color::new(0.2, 0.3, 0.1),
    ));
    let mut objects = HittableList::new();
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Lambertian::new_arc(checker.clone()),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Lambertian::new_arc(checker),
    )));
    objects
}
fn two_perlin_spheres() -> HittableList {
    let mut obj = HittableList::new();
    let pertext = Arc::new(NoiseTexture::new(4.0));
    obj.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new_arc(pertext.clone()),
    )));
    obj.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Lambertian::new_arc(pertext),
    )));
    obj
}
fn earth() -> HittableList {
    let img = image::open("earthmap.jpg")
        .expect("Could not find the image")
        .to_rgb8();
    let (width, height) = img.dimensions();
    let data = img.as_raw();
    let earth_texture = ImageTexture::new(data.clone(), width as usize, height as usize);
    let mut obj = HittableList::new();
    let earth_surface = Lambertian::new_arc(Arc::new(earth_texture));
    obj.add(Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        2.0,
        earth_surface,
    )));
    obj
}
fn simple_light() -> HittableList {
    let mut obj = HittableList::new();
    let pertext = Arc::new(NoiseTexture::new(4.0));
    obj.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new_arc(pertext.clone()),
    )));
    obj.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Lambertian::new_arc(pertext),
    )));
    let difflight = Arc::new(DiffuseLight::new_color(Color::new(4.0, 4.0, 4.0)));
    obj.add(Arc::new(XYRect::new(difflight, 3.0, 5.0, 1.0, 3.0, -2.0)));
    obj.add(Arc::new(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        DiffuseLight::new_color(Color::new(4.0, 4.0, 4.0)),
    )));
    obj
}
fn main() {
    // get environment variable CI, which is true for GitHub Actions
    let is_ci = is_ci();

    println!("CI: {}", is_ci);

    let path = "output/test.jpg";
    let quality = 60; // From 0 to 100, suggested value: 60

    //image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    let mut samples_per_pixel: usize = 100;
    const MAX_DEPTH: usize = 50;
    let width = IMAGE_WIDTH;
    let height = IMAGE_HEIGHT;

    //world
    let obj;
    let lookfrom;
    let lookat;
    let vfov;
    let mut aperture = 0.0;
    let background;
    let mode = 0;
    match mode {
        1 => {
            obj = random_scene();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            background = Color::new(0.7, 0.8, 1.0);
            vfov = 20.0;
            aperture = 0.1;
        }
        2 => {
            obj = two_spheres();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            background = Color::new(0.7, 0.8, 1.0);
            vfov = 20.0;
        }
        3 => {
            obj = two_perlin_spheres();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            background = Color::new(0.7, 0.8, 1.0);
            vfov = 20.0;
        }
        4 => {
            obj = earth();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            background = Color::new(0.7, 0.8, 1.0);
            vfov = 20.0;
        }
        _ => {
            obj = simple_light();
            samples_per_pixel = 400;
            background = Color::zero();
            lookfrom = Point3::new(26.0, 3.0, 6.0);
            lookat = Point3::new(0.0, 2.0, 0.0);
            vfov = 20.0;
        }
    }
    let world = BvhNode::newnew(obj, 0.0, 1.0);

    //camera
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    // Create image data
    let mut img: RgbImage = ImageBuffer::new(width.try_into().unwrap(), height.try_into().unwrap());

    // Progress bar UI powered by library `indicatif`
    // You can use indicatif::ProgressStyle to make it more beautiful
    // You can also use indicatif::MultiProgress in multi-threading to show progress of each thread
    let bar = if is_ci {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    for j in 0..height {
        for i in 0..width {
            let mut color = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let u = (i as f64 + random_f64()) / ((width - 1) as f64);
                let v = (j as f64 + random_f64()) / ((height - 1) as f64);
                let r = cam.get_ray(u, v);
                color += ray_color(r, &background, &*world, MAX_DEPTH as i32);
            }
            let scale = 1.0 / samples_per_pixel as f64;
            let r = (color.x() * scale).sqrt();
            let g = (color.y() * scale).sqrt();
            let b = (color.z() * scale).sqrt();
            let pixel_color = [
                (256.0 * clamp(r, 0.0, 0.999)) as u8,
                (256.0 * clamp(g, 0.0, 0.999)) as u8,
                (256.0 * clamp(b, 0.0, 0.999)) as u8,
            ];

            write_color(pixel_color, &mut img, i, height - j - 1);
            bar.inc(1);
        }
    }

    // Finish progress bar
    bar.finish();

    // Output image to file
    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}
