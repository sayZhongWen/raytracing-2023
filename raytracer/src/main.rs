extern crate core;

mod aabb;
mod aarect;
mod r#box;
mod bvh;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod medium;
mod perlin;
mod ray;
mod rtweekend;
mod sphere;
mod texture;
mod vec3;

use crate::bvh::BvhNode;
use crate::camera::Camera;
use crate::hittable::{Hit, RotateY, Translate};
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal};
pub use crate::ray::Ray;
use crate::rtweekend::*;
use crate::sphere::{MovingSphere, Sphere};
use color::write_color;

use crate::aarect::{XYRect, XZRect, YZRect};
use crate::medium::ConstantMedium;
use crate::r#box::Bbox;
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use crate::vec3::{Color, Point3};
use image::ImageBuffer;
use indicatif::ProgressBar;
use std::fs::File;
// use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;

// use threadpool::ThreadPool;
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
    let earth_texture = ImageTexture::new();
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
fn cornell_box() -> HittableList {
    let mut obj = HittableList::new();
    let red = Arc::new(Lambertian::new_color(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_color(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_color(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_color(Color::new(15.0, 15.0, 15.0)));
    obj.add(Arc::new(YZRect::new(green, 0.0, 555.0, 0.0, 555.0, 555.0)));
    obj.add(Arc::new(YZRect::new(red, 0.0, 555.0, 0.0, 555.0, 0.0)));
    obj.add(Arc::new(XZRect::new(
        light, 213.0, 343.0, 227.0, 332.0, 554.0,
    )));
    obj.add(Arc::new(XZRect::new(
        white.clone(),
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
    )));
    obj.add(Arc::new(XZRect::new(
        white.clone(),
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
    )));
    obj.add(Arc::new(XYRect::new(
        white.clone(),
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
    )));
    let box1 = Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(Bbox::new(
                &Point3::zero(),
                &Point3::new(165.0, 330.0, 165.0),
                white.clone(),
            )),
            15.0,
        )),
        Vec3::new(265.0, 0.0, 295.0),
    ));
    obj.add(box1);

    let box2 = Arc::new(Bbox::new(
        &Point3::zero(),
        &Point3::new(165.0, 165.0, 165.0),
        white,
    ));
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    obj.add(box2);
    obj
}
fn cornell_smoke() -> HittableList {
    let mut obj = HittableList::new();
    let red = Arc::new(Lambertian::new_color(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_color(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_color(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_color(Color::new(15.0, 15.0, 15.0)));
    obj.add(Arc::new(YZRect::new(green, 0.0, 555.0, 0.0, 555.0, 555.0)));
    obj.add(Arc::new(YZRect::new(red, 0.0, 555.0, 0.0, 555.0, 0.0)));
    obj.add(Arc::new(XZRect::new(
        light, 213.0, 343.0, 227.0, 332.0, 554.0,
    )));
    obj.add(Arc::new(XZRect::new(
        white.clone(),
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
    )));
    obj.add(Arc::new(XZRect::new(
        white.clone(),
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
    )));
    obj.add(Arc::new(XYRect::new(
        white.clone(),
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
    )));
    let box1 = Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(Bbox::new(
                &Point3::zero(),
                &Point3::new(165.0, 330.0, 165.0),
                white.clone(),
            )),
            15.0,
        )),
        Vec3::new(265.0, 0.0, 295.0),
    ));

    let box2 = Arc::new(Bbox::new(
        &Point3::zero(),
        &Point3::new(165.0, 165.0, 165.0),
        white,
    ));
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    obj.add(Arc::new(ConstantMedium::new_color(
        box1,
        0.01,
        Color::zero(),
    )));
    obj.add(Arc::new(ConstantMedium::new_color(
        box2,
        0.01,
        Color::ones(),
    )));
    obj
}
fn final_scene() -> HittableList {
    let mut boxes1 = HittableList::new();
    let ground = Arc::new(Lambertian::new_color(Color::new(0.48, 0.83, 0.53)));
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random(1.0, 101.0);
            let z1 = z0 + w;
            boxes1.add(Arc::new(Bbox::new(
                &Point3::new(x0, y0, z0),
                &Point3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }
    let mut obj = HittableList::new();
    obj.add(BvhNode::newnew(boxes1, 0.0, 1.0));
    let light = Arc::new(DiffuseLight::new_color(Color::new(7.0, 7.0, 7.0)));
    obj.add(Arc::new(XZRect::new(
        light, 123.0, 423.0, 147.0, 412.0, 554.0,
    )));
    let center1 = Point3::new(400.0, 400.0, 400.0);
    let center2 = center1.clone() + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Lambertian::new_color(Color::new(0.7, 0.3, 0.1));
    obj.add(Arc::new(MovingSphere::new(
        center1,
        center2,
        0.0,
        1.0,
        50.0,
        moving_sphere_material,
    )));
    obj.add(Arc::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Dielectric::new(1.5),
    )));
    obj.add(Arc::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Metal::new(&Color::new(0.8, 0.8, 0.8), 1.0),
    )));
    let mut boundary = Arc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Dielectric::new(1.05),
    ));
    obj.add(boundary.clone());
    obj.add(Arc::new(ConstantMedium::new_color(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));
    boundary = Arc::new(Sphere::new(Point3::zero(), 5000.0, Dielectric::new(1.5)));
    obj.add(Arc::new(ConstantMedium::new_color(
        boundary,
        0.0001,
        Color::ones(),
    )));
    let emat = Lambertian::new_arc(Arc::new(ImageTexture::new()));
    obj.add(Arc::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));
    let pertext = Arc::new(NoiseTexture::new(0.1));
    obj.add(Arc::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Lambertian::new_arc(pertext),
    )));
    let mut boxes2 = HittableList::new();
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Arc::new(Sphere::new(
            Point3::random(0.0, 165.0),
            10.0,
            Lambertian::new_color(Color::new(0.73, 0.73, 0.73)),
        )));
    }
    obj.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(BvhNode::newnew(boxes2, 0.0, 1.0), 15.0)),
        Vec3::new(-100.0, 270.0, 395.0),
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
    // const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    let mut samples_per_pixel: usize = 100;
    const MAX_DEPTH: usize = 50;
    let mut width = IMAGE_WIDTH;
    let mut aspect_ratio = ASPECT_RATIO;

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
        5 => {
            obj = simple_light();
            samples_per_pixel = 400;
            background = Color::zero();
            lookfrom = Point3::new(26.0, 3.0, 6.0);
            lookat = Point3::new(0.0, 2.0, 0.0);
            vfov = 20.0;
        }
        6 => {
            obj = cornell_box();
            aspect_ratio = 1.0;
            width = 600;
            samples_per_pixel = 200;
            background = Color::zero();
            lookfrom = Point3::new(278.0, 278.0, -800.0);
            lookat = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        7 => {
            obj = cornell_smoke();
            aspect_ratio = 1.0;
            width = 600;
            samples_per_pixel = 200;
            background = Color::zero();
            lookfrom = Point3::new(278.0, 278.0, -800.0);
            lookat = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        _ => {
            obj = final_scene();
            aspect_ratio = 1.0;
            width = 800;
            // width = 300;
            // samples_per_pixel = 10000;
            samples_per_pixel = 5000;
            background = Color::zero();
            lookfrom = Point3::new(478.0, 278.0, -600.0);
            lookat = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
    }
    let world = BvhNode::newnew(obj, 0.0, 1.0);
    let height = (width as f64 / aspect_ratio) as usize;
    //camera
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    // Create image data
    // let mut img = ImageBuffer::new(width.try_into().unwrap(), height.try_into().unwrap());

    // Progress bar UI powered by library `indicatif`
    // You can use indicatif::ProgressStyle to make it more beautiful
    // You can also use indicatif::MultiProgress in multi-threading to show progress of each thread

    // let sections = 20;
    // let workers = 20; //20,20 2min38s
    // let bar = if is_ci {
    //     ProgressBar::hidden()
    // } else {
    //     ProgressBar::new((height * width) as u64)
    //     // ProgressBar::new(sections as u64)
    // };
    let bar = if is_ci {
        Arc::new(ProgressBar::hidden())
    } else {
        Arc::new(ProgressBar::new((height * width) as u64))
    };
    // for j in 0..height {
    //     for i in 0..width {
    //         let mut color = Vec3::new(0.0, 0.0, 0.0);
    //         for _s in 0..samples_per_pixel {
    //             let u = (i as f64 + random_f64()) / ((width - 1) as f64);
    //             let v = (j as f64 + random_f64()) / ((height - 1) as f64);
    //             let r = cam.get_ray(u, v);
    //             color += ray_color(r, &background, &*world, MAX_DEPTH as i32);
    //         }
    //         let scale = 1.0 / samples_per_pixel as f64;
    //         let r = (color.x() * scale).sqrt();
    //         let g = (color.y() * scale).sqrt();
    //         let b = (color.z() * scale).sqrt();
    //         let pixel_color = [
    //             (256.0 * clamp(r, 0.0, 0.999)) as u8,
    //             (256.0 * clamp(g, 0.0, 0.999)) as u8,
    //             (256.0 * clamp(b, 0.0, 0.999)) as u8,
    //         ];
    //
    //         write_color(pixel_color, &mut img, i, height - j - 1);
    //         bar.inc(1);
    //     }
    // }

    //方法一：mpsc
    // let (sender, receiver) = channel();
    // let pool = ThreadPool::new(workers);
    // for t in 0..sections {
    //     let sender = sender.clone();
    //     let worldd = world.clone();
    //     let camm = cam.clone();
    //     let bg = background.clone();
    //     pool.execute(move || {
    //         let begin = height * t / sections;
    //         let end = height * (t + 1) / sections;
    //         let mut res = ImageBuffer::new(width as u32, height as u32 / sections as u32);
    //         for (img_j, j) in (begin..end).enumerate() {
    //             for i in 0..width {
    //                 let mut color = Vec3::new(0.0, 0.0, 0.0);
    //                 for _s in 0..samples_per_pixel {
    //                     let u = (i as f64 + random_f64()) / ((width - 1) as f64);
    //                     let v = (j as f64 + random_f64()) / ((height - 1) as f64);
    //                     let r = camm.get_ray(u, v);
    //                     color += ray_color(r, &bg, &*worldd, MAX_DEPTH as i32);
    //                 }
    //                 let scale = 1.0 / samples_per_pixel as f64;
    //                 let r = (color.x() * scale).sqrt();
    //                 let g = (color.y() * scale).sqrt();
    //                 let b = (color.z() * scale).sqrt();
    //                 let pixel_color = [
    //                     (256.0 * clamp(r, 0.0, 0.999)) as u8,
    //                     (256.0 * clamp(g, 0.0, 0.999)) as u8,
    //                     (256.0 * clamp(b, 0.0, 0.999)) as u8,
    //                 ];
    //                 write_color(pixel_color, &mut res, i, img_j);
    //             }
    //         }
    //         sender
    //             .send((begin..end, res))
    //             .expect("Fail to send the result!");
    //     });
    // }
    // let mut img = ImageBuffer::new(width as u32, height as u32);
    // for (rows, data) in receiver.iter().take(sections) {
    //     for (idx, row) in rows.enumerate() {
    //         for col in 0..width {
    //             *img.get_pixel_mut(col as u32, (height - row - 1) as u32) =
    //                 *data.get_pixel(col as u32, idx as u32);
    //             bar.inc(1);
    //         }
    //     }
    // }

    //方法二：Arc+Mutex
    let img = Arc::new(Mutex::new(ImageBuffer::new(
        width.try_into().unwrap(),
        height.try_into().unwrap(),
    )));
    let mut handles = vec![];
    let thread_number = 20;
    for t in 0..thread_number {
        let world = Arc::clone(&world);
        let img = Arc::clone(&img);
        let bar = Arc::clone(&bar);
        let bg = background.clone();
        let camm = cam.clone();
        let handle = thread::spawn(move || {
            for j in (t * height / thread_number)..((t + 1) * height / thread_number) {
                for i in 0..width {
                    let mut color = Color::zero();
                    for _s in 0..samples_per_pixel {
                        let u = (i as f64 + random_f64()) / (width - 1) as f64;
                        let v = (j as f64 + random_f64()) / (height - 1) as f64;
                        let r = camm.get_ray(u, v);
                        color += ray_color(r, &bg, &*world, MAX_DEPTH as i32);
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
                    write_color(pixel_color, &mut img.lock().unwrap(), i, height - j - 1);
                    bar.inc(1);
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    // Finish progress bar
    bar.finish();

    // Output image to file
    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    // let output_image = image::DynamicImage::ImageRgb8(img);
    let output_image =
        image::DynamicImage::ImageRgb8(Mutex::into_inner(Arc::into_inner(img).unwrap()).unwrap());
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}
