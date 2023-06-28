mod aabb;
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
use crate::material::{Dielectric, Lambertian, Metal};
pub use crate::ray::Ray;
use crate::rtweekend::*;
use crate::sphere::{MovingSphere, Sphere};
use color::write_color;

use crate::texture::{CheckerTexture, NoiseTexture};
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

fn ray_color(r: Ray, world: &dyn Hit, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }
    if let Some(rec) = world.hit(&r, 0.001, f64::INFINITY) {
        return if let Some((scattered, attenuation)) = rec.material.scatter(&r, &rec) {
            attenuation * ray_color(scattered, world, depth - 1)
        } else {
            Vec3::zero()
        };
    }
    let unit_direction: Vec3 = r.dir().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
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
    let pertext = Arc::new(NoiseTexture::new());
    // let pertext2 = Arc::new(NoiseTexture::new());
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
    const SAMPLES_PER_PIXEL: usize = 100;
    const MAX_DEPTH: usize = 50;
    let width = IMAGE_WIDTH;
    let height = IMAGE_HEIGHT;

    //world
    let mut obj = HittableList::new();
    let mut lookfrom = Point3::zero();
    let mut lookat = Point3::zero();
    let mut vfov = 40.0;
    let mut aperture = 0.0;
    let mode = 0;
    match mode {
        1 => {
            obj = random_scene();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.1;
        }
        2 => {
            obj = two_spheres();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        _ => {
            obj = two_perlin_spheres();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
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
            for _s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_f64()) / ((width - 1) as f64);
                let v = (j as f64 + random_f64()) / ((height - 1) as f64);
                let r = cam.get_ray(u, v);
                color += ray_color(r, &*world, MAX_DEPTH as i32);
            }
            let scale = 1.0 / SAMPLES_PER_PIXEL as f64;
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
