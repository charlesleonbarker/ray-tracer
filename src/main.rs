use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{Error, Write};


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


    for j in 0..image_height{
        for i in 0..image_width{
            let r = i as f64/(image_width as f64 - 1.0);
            let g =  j as f64/(image_height as f64 - 1.0);
            let b = 0.25;

            let ir:i64 = (255.999*r) as i64;
            let ig = (255.999*g) as i64;
            let ib = (255.999*b) as i64;

            write!(file, "{} {} {}\n", ir, ig, ib);
        }
    }
}