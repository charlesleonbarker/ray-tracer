#[macro_use] 

extern crate impl_ops;
extern crate fastrand;

mod vec;
mod ray;
mod sphere;
mod traceable;
mod camera;
mod material;

use crate::vec::*;
use crate::ray::*;
use crate::sphere::*;
use crate::traceable::*;
use crate::camera::*;
use crate::material::*;


use std::f64::INFINITY as infinity;
use std::f64::consts::PI as pi;
use std::fs::OpenOptions;
use std::io::Write;

pub fn deg_to_rad(deg:f64) -> f64{
    deg*pi/180.0
}

//Generates random numbers between [min_inc, max_exc)
pub fn rand_double(min_inc: f64, max_exc: f64) -> f64{
    fastrand::f64()*(max_exc - min_inc) + min_inc
}

pub fn bound(x: f64, min: f64, max:f64) -> f64{
    if x < min{return min}
    if x > max{return max}
    x
}

pub fn ray_color(r: &Ray, world: &TraceableList, depth: i32) -> Color {
    let mut rec = HitRecord::default();

    //If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0{
        return Color::new(0.0,0.0,0.0)
    }

    let result = world.trace(r, 0.001, infinity, &mut rec);
    match result{
        TraceResult::Scattered((attenuation, scattered)) => return attenuation.elementwise_mult(&ray_color(&scattered, world, depth-1)),
        TraceResult::Absorbed => return Color::new(0.0, 0.0, 0.0),
        TraceResult::Missed =>{
            let unit_dir = r.direction().unit_vector();
            let t = 0.5*(unit_dir.y() + 1.0);
            return (1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0)
        }         
    }
}

fn main(){

    //Image
    const IMAGE_WIDTH:i32 = 600;
    const IMAGE_HEIGHT:i32 = ((IMAGE_WIDTH as f64)/ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    //World
    let mut world = TraceableList::new();
    let mat_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let mat_center = Lambertian::new(Color::new(0.2, 0.3, 0.5));
    let mat_left = Dielectric::new(1.5);
    let mat_right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.7);

    let ground = Sphere::new(&Point3::new(0.0,-100.5,-1.0), 100.0, &mat_ground);
    let sphere_center = Sphere::new(&Point3::new(0.0,0.0,-1.0), 0.5, &mat_center);
    let sphere_left = Sphere::new(&Point3::new(-1.0,0.0,-1.0), 0.5, &mat_left);
    let sphere_right = Sphere::new(&Point3::new(1.0,0.0,-1.0), 0.5, &mat_right);
    
    world.add(&ground);
    world.add(&sphere_center);
    world.add(&sphere_left);
    world.add(&sphere_right);


    //Camera
    let cam = Camera::new();

    //Render
    let path = "results.ppm";
    let mut file = OpenOptions::new()
                                    .create(true)
                                    .write(true)
                                    .open(path)
                                    .unwrap();

    write!(file, "P3\n{} {} \n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).unwrap();
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
                let u = (rand_double(0.0, 1.0) + i as f64)/(IMAGE_WIDTH as f64 - 1.0);
                let v = (rand_double(0.0, 1.0) + (IMAGE_HEIGHT - j) as f64)/((IMAGE_HEIGHT - 1) as f64);
                let r = cam.get_ray(u,v);
                pixel_color = pixel_color + ray_color(&r, &mut world, MAX_DEPTH);
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
