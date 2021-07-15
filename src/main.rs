#[macro_use] 
extern crate impl_ops;
extern crate rand;

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{Error, Write};
mod vec;
use crate::vec::*;
mod ray;
use crate::ray::*;
mod sphere;
use crate::sphere::*;
mod hittable;
use crate::hittable::*;
use std::f64::INFINITY as infinity;
use std::f64::consts::PI as pi;

pub fn deg_to_rad(deg:f64) -> f64{
    deg*180.0/pi
}

pub fn rand_double() -> f64{
    rand::random()
}

pub fn rand_double_in(min: f64, max: f64) -> f64{
    rand::random::<f64>()*(max - min) + min
}

pub fn ray_color(r: &Ray, world: &mut HittablesList) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(r, 0.0, infinity, &mut rec){
        0.5*(rec.normal + Color::new(1.0, 1.0, 1.0))
    }else {
        let unit_dir = r.direction().unit_vector();
        let t = 0.5*(unit_dir.y() + 1.0);
        (1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0)
    }
}

fn main(){

    //Image
    const aspect_ratio: f64 = 16.0/9.0;
    const image_width:i32 = 256;
    const image_height:i32 = ((image_width as f64)/aspect_ratio) as i32;

    //World
    let mut world = HittablesList::new();
    let s1 = Sphere::new(&Point3::new(0.0,0.0,-1.0), 0.5);
    let ground = Sphere::new(&Point3::new(0.0,-100.5,-1.0), 100.0);
    world.add(&s1);
    world.add(&ground);

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
            let pixel_color = ray_color(&r, &mut world);
            pixel_color.write_color(&mut file);
        }
    }
}