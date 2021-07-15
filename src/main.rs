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
mod camera;
use crate::camera::*;

pub fn deg_to_rad(deg:f64) -> f64{
    deg*pi/180.0
}

pub fn rand_double() -> f64{
    rand::random()
}

pub fn rand_double_in(min: f64, max: f64) -> f64{
    rand::random::<f64>()*(max - min) + min
}

pub fn bound(x: f64, min: f64, max:f64) -> f64{
    if x < min{return min}
    if x > max{return max}
    x
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
    const IMAGE_WIDTH:i32 = 400;
    const IMAGE_HEIGHT:i32 = ((IMAGE_WIDTH as f64)/ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 10;

    //World
    let mut world = HittablesList::new();
    let s1 = Sphere::new(&Point3::new(0.0,0.0,-1.0), 0.5);
    let ground = Sphere::new(&Point3::new(0.0,-100.5,-1.0), 100.0);
    world.add(&s1);
    world.add(&ground);

    //Camera
    let cam = Camera::new();

    //Render
    let path = "results.ppm";
    let mut file = OpenOptions::new()
                                    .create(true)
                                    .write(true)
                                    .open(path)
                                    .unwrap();

    write!(file, "P3\n{} {} \n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    let mut perc:i32 = 0;
    println!("{}",IMAGE_WIDTH*IMAGE_HEIGHT);
    for j in 0..IMAGE_HEIGHT{
        // Loading Bar Output
        if perc != ((j as f64)/(IMAGE_HEIGHT as f64)*100.0) as i32 {
            perc = ((j as f64)/(IMAGE_HEIGHT as f64)*100.0) as i32;
            println!("{}", ((j as f64)/(IMAGE_HEIGHT as f64)*100.0) as i32);
        }
        for i in 0..IMAGE_WIDTH{
            let mut pixel_color = Color::new(0.0,0.0,0.0);
            for _ in 0..SAMPLES_PER_PIXEL{
                let u = (rand_double() + i as f64)/(IMAGE_WIDTH as f64 - 1.0);
                let v = (rand_double() + (IMAGE_HEIGHT - j) as f64)/((IMAGE_HEIGHT - 1) as f64);
                let r = cam.get_ray(u,v);
                pixel_color = pixel_color + ray_color(&r, &mut world);
            }
            pixel_color.write_color(&mut file, SAMPLES_PER_PIXEL);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bound(){
        let x = 10.0;
        let max_x = bound(x, 5.0, 7.0);
        let min_x = bound(x, 11.0, 14.0);
        assert_eq!(max_x, 7.0);
        assert_eq!(min_x, 11.0);
    }

    #[test]
    fn test_deg_2_rad(){
        let deg = 180.0;
        let rad = deg_to_rad(deg);
        assert_eq!(pi, rad);
    }

}
