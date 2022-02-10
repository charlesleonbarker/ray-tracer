#[macro_use] 

extern crate impl_ops;
extern crate fastrand;
extern crate tobj;

mod vec;
mod ray;
mod sphere;
mod traceable;
mod camera;
mod material;
mod util;
mod bvh;
mod rect;
mod triangle;

use bvh::BvhNode;

use crate::vec::*;
use crate::ray::*;
use crate::sphere::*;
use crate::traceable::*;
use crate::camera::*;
use crate::material::*;
use crate::rect::*;
use crate::util::*;
use crate::triangle::*;

use std::f64::INFINITY;
use std::fs::OpenOptions;
use std::io::Write;

pub fn ray_color(r: &Ray, background: Color, world: &dyn Traceable, depth: i32) -> Color {

    //If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0{
        return Color::new(0.0,0.0,0.0)
    }

    let result = world.trace(r, 0.001, INFINITY);
    match result{
        TraceResult::Scattered((attenuation, scattered)) => attenuation.elementwise_mult(&ray_color(&scattered, background, world, depth-1)),
        TraceResult::Absorbed(emitted) => emitted,
        TraceResult::Missed => background      
    }
}

pub fn sphere_world() -> TraceableList{
    let mut world = TraceableList::new();
    let mat_ground = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    let ground = Box::new(Sphere::new(Point3::new(0.0,-1000.0,0.0), 1000.0, mat_ground));
    world.add(ground);

    for a in -11..12{
        for b in -11..12{
            let choose_mat = rand_double(0.0, 1.0);
            let center = Point3::new(a as f64 + 0.9*rand_double(0.0, 1.0), 0.2, b as f64 + 0.9*rand_double(0.0, 1.0));

            if choose_mat < 0.6{
                let albedo = Color::rand(0.0, 1.0).elementwise_mult(&Color::rand(0.0, 1.0));
                let sphere_material = Lambertian::new(albedo);
                let sphere = Box::new(Sphere::new(center, 0.2, sphere_material));
                world.add(sphere);
            } else if choose_mat < 0.9{
                let albedo = Color::rand(0.5, 1.0);
                let fuzz = rand_double(0.0, 0.5);
                let sphere_material = Metal::new(albedo, fuzz);
                let sphere = Box::new(Sphere::new(center, 0.2, sphere_material));
                world.add(sphere);
            } else {
                let sphere_material = Dielectric::new(1.5);
                let sphere = Box::new(Sphere::new(center, 0.2, sphere_material));
                world.add(sphere);
            }
        }
    }
    let mat_center = Dielectric::new(1.5);
    let mat_left = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    let mat_right = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);

    let sphere_center = Box::new(Sphere::new(Point3::new(0.0,1.0,0.0), 1.0, mat_center));
    let sphere_left = Box::new(Sphere::new(Point3::new(-4.0,1.0,0.0), 1.0, mat_left));
    let sphere_right = Box::new(Sphere::new(Point3::new(4.0,1.0,0.0), 1.0, mat_right));
    

    world.add(sphere_center);
    world.add(sphere_left);
    world.add(sphere_right);
    world
}

pub fn light_test() -> TraceableList{
    let mut world = TraceableList::new();
    let mat = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    let ground = Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, mat));
    let sphere = Box::new(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, Lambertian::new(Color::new(0.8, 0.8, 0.8)))); 

    let diff_light = DiffuseLights::new(Color::new(4.0,4.0,4.0));
    let rect = Box::new(Rect::new(RectAxes::XY, -1.0, 2.0, 1.0, 3.0, 4.0, diff_light));
    world.add(ground);
    world.add(sphere);
    //world.add(rect);
    world
}

pub fn triangle_test() -> TraceableList{
    let mut world = TraceableList::new();
    let mat = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    let ground = Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, mat));
 
    let mat = Lambertian::new(Vec3::new(0.8, 0.8, 0.8));
    let v0 = Vec3::new(-2.0, 0.1, 0.0);
    let v1 = Vec3::new(2.0, 0.1, 0.0);
    let v2 = Vec3::new(0.0, 2.1, 0.0);
    let norms = [Vec3::new(0.0, 0.0, 1.0); 3];
    let tri = Box::new(Triangle::new([v0, v1, v2], norms, mat));
    world.add(ground);
    world.add(tri);
    world
}

pub fn obj_test() -> TraceableList{
    let mut world = TraceableList::new(); 
    let mut mesh = TraceableList::new(); 
    let mat = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    let ground = Box::new(Sphere::new(Point3::new(0.0, -1050.0, 0.0), 1000.0, mat));
    let (mut models, materials) = import_obj("/Users/simonpapworth/Downloads/helicopter.obj");
    let diff_light = DiffuseLights::new(Color::new(4.0,4.0,4.0));
    let rect = Box::new(Rect::new(RectAxes::XY, -4.0, -2.0, 1.0, 8.0, 4.0, diff_light));
    mesh.add_obj(models, materials);
    world.add(ground);
    world.add(Box::new(mesh.to_Bvh()));
    world
}

pub fn mesh_test() -> TraceableList{
    let mut world = TraceableList::new(); 
    let mut mesh_1 = tobj::Mesh::default();
    let mut mesh_2 = tobj::Mesh::default();
    let mut mesh_3 = tobj::Mesh::default();

    mesh_1.positions = vec!(-2.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 1.0, 0.0,
                            0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 1.0, 1.0, 0.0,
                            2.0, 0.0, 0.0, 4.0, 0.0, 0.0, 3.0, 1.0, 0.0);
                            
    mesh_1.normals = vec!(0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
                          0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
                          0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0);

    mesh_1.indices = vec!(0, 1, 2, 3, 4, 5, 6, 7, 8);


    mesh_2.positions = vec!(-2.0, 1.0, 0.0, 0.0, 1.0, 0.0, -1.0, 2.0, 0.0,
                            0.0, 1.0, 0.0, 2.0, 1.0, 0.0, 1.0, 2.0, 0.0,
                            2.0, 1.0, 0.0, 4.0, 1.0, 0.0, 3.0, 2.0, 0.0);

    mesh_2.normals = vec!(0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
                        0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
                        0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0);

    mesh_2.indices = vec!(0, 1, 2, 3, 4, 5, 6, 7, 8);
                    

    mesh_3.positions = vec!(-2.0, 2.0, 0.0, 0.0, 2.0, 0.0, -1.0, 3.0, 0.0,
                            0.0, 2.0, 0.0, 2.0, 2.0, 0.0, 1.0, 3.0, 0.0,
                            2.0, 2.0, 0.0, 4.0, 2.0, 0.0, 3.0, 3.0, 0.0);

    mesh_3.normals = vec!(0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
                          0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
                          0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0);

    mesh_3.indices = vec!(0, 1, 2, 3, 4, 5, 6, 7, 8);


    let test_1 = tobj::Model::new(mesh_1, "test_1".to_string());
    let test_2 = tobj::Model::new(mesh_2, "test_1".to_string());
    let test_3 = tobj::Model::new(mesh_3, "test_1".to_string());


    let test = vec!(test_1, test_2, test_3);
    world.add_obj(test, None);
    world
}

fn main(){

    //Image
    const IMAGE_WIDTH:i32 = 400;
    const IMAGE_HEIGHT:i32 = ((IMAGE_WIDTH as f64)/ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 200;
    const MAX_DEPTH: i32 = 50;



    let background: Color;
    let world: TraceableList;
    let look_from: Vec3;
    let look_at: Vec3;

    let scene = 3;

    match scene{
        0 => {
            background = Color::new(0.7, 0.8, 1.0);
            world = sphere_world();
            look_from = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::new(0.0, 0.0, 0.0);
        }

        1 => {
            background = Color::new(0.9, 0.9, 0.9);
            world = light_test();
            look_from = Point3::new(26.0, 3.0, 6.0);
            look_at = Point3::new(0.0, 2.0, 0.0);
        }

        2 =>{
            background = Color::new(0.9, 0.9, 0.9);
            world = triangle_test();
            look_from = Point3::new(0.0, 2.0, 26.0);
            look_at = Point3::new(0.0, 0.0, 0.0);
        }

        3 =>{
            background = Color::new(0.9, 0.9, 0.9);
            world = obj_test();
            look_from = Point3::new(-40.0, 10.0, 40.0);
            //look_from = Point3::new(0.8, 0.2, 1.0);
            look_at = Point3::new(0.0, 0.0, 0.0);
        }

        4 =>{
            background = Color::new(0.9, 0.9, 0.9);
            world = mesh_test();
            look_from = Point3::new(26.0, 10.0, 10.0);
            //look_from = Point3::new(0.8, 0.2, 1.0);
            look_at = Point3::new(0.0, 0.0, 0.0);
        }

        _ => panic!("Scene ID invalid.")
    }
    
    //let world = world.to_Bvh();

    //Camera

    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let cam = Camera::new(look_from, look_at, v_up, 20.0, ASPECT_RATIO, aperture, dist_to_focus);

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
                pixel_color = pixel_color + ray_color(&r, background, &world, MAX_DEPTH);
            }
            pixel_color.write_color(&mut file, SAMPLES_PER_PIXEL);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
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
        assert_eq!(PI, rad);
    }

}
