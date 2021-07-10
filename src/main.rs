use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{Error, Write};
mod vec;
use crate::vec::*;
mod ray;
fn main(){

    let image_width = 256;
    let image_height = 256;

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
        // Write to Image
        for i in 0..image_width{
            let pixel_color = Color::new(i as f64/(image_width as f64 - 1.0),
                                                    (image_height - j) as f64/(image_height as f64 - 1.0),
                                                    0.25);
            pixel_color.write_color(&mut file);
        }
    }
}