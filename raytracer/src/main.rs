mod color;
mod vec3;
mod ray;

use color::write_color;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::fs::File;
pub use vec3::Vec3;
pub use crate::ray::Ray;

const AUTHOR: &str = "Dizzy_D";

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn ray_color(r: &Ray) -> Vec3{
    let unit_direction: Vec3 = r.dir().unit_vector();
    let t=0.5*(unit_direction.y()+1.0);
    (1.0-t)*Vec3::new(1.0,1.0,1.0)+t*Vec3::new(0.5,0.7,1.0)
}

fn main() {
    // get environment variable CI, which is true for GitHub Actions
    let is_ci = is_ci();

    println!("CI: {}", is_ci);

    // let height: usize = 800;
    // let width: usize = 800;
    let path = "output/test.jpg";
    let quality = 60; // From 0 to 100, suggested value: 60

    //image
    const ASPECT_RATIO: f64 = 16.0/9.0;
    const width: usize = 400;
    const height: usize = 225;
    //camera
    let viewpoint_height:f64=2.0;
    let viewpoint_width=viewpoint_height*ASPECT_RATIO;
    let focal_length=1.0;
    let origin=Vec3::new(0.0,0.0,0.0);
    let horizontal=Vec3::new(viewpoint_width,0.0,0.0);
    let vertical=Vec3::new(0.0,viewpoint_height,0.0);
    let lower_left_corner=origin.clone()-horizontal.clone()/2.0-vertical.clone()/2.0-Vec3::new(0.0,0.0,focal_length);

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
            let u=i as f64/(width-1) as f64;
            let v=j as f64/(height-1) as f64;
            let r=Ray::new(origin.clone(),lower_left_corner.clone()+horizontal.clone()*u+vertical.clone()*v-origin.clone());
            let pixel_color=[
                (ray_color(&r).x()*255.0).floor() as u8,
                (ray_color(&r).y()*255.0).floor() as u8,
                (ray_color(&r).z()*255.0).floor() as u8,
            ];
            // let pixel_color = [
            //     (j as f32 / height as f32 * 255.).floor() as u8,
            //     ((i + height - j) as f32 / (height + width) as f32 * 255.).floor() as u8,
            //     (i as f32 / height as f32 * 255.).floor() as u8,
            // ];
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
