use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{Error, Write};
mod vec;
use crate::vec::*;
mod ray;
use crate::ray::*;

pub fn ray_color(r: Ray) -> Color{
    let unit_dir = r.direction().unit_vector();
    let t = 0.5*(unit_dir.y() + 1.0);
    (1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0)
}

fn main(){

    //Image
    const aspect_ratio: f64 = 16.0/9.0;
    const image_width:i32 = 256;
    const image_height:i32 = ((image_width as f64)/aspect_ratio) as i32;

    //Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio*viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0,0.0 );
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);

    //Render
    let path = "results.ppm";
    let mut file = OpenOptions::new()
                                    .create(true)
                                    .write(true)
                                    .open(path)
                                    .unwrap();

    write!(file, "P3\n{} {} \n255\n", image_width, image_height);
    let mut perc:i32 = 0;

    for j in 0..image_height{
        // Loading Bar Output
        if perc != ((j as f64)/(image_height as f64)*100.0) as i32 {
            perc = ((j as f64)/(image_height as f64)*100.0) as i32;
            println!("{}", ((j as f64)/(image_height as f64)*100.0) as i32);
        }
        for i in 0..image_width{
            let u = i as f64/(image_width as f64 - 1.0);
            let v = ((image_height - j) as f64)/((image_height - 1) as f64);
            let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            let pixel_color = ray_color(r);
            pixel_color.write_color(&mut file);
        }
    }
}