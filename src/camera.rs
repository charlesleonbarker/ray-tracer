use core::f64;

use crate::deg_to_rad;
use crate::vec::*;
use crate::ray::*;

pub const ASPECT_RATIO: f64 = 16.0/9.0;

#[derive (Copy, Clone, Default)]
pub struct Camera{
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera{

    pub fn new(look_from: Point3, look_at: Point3, v_up: Vec3, v_fov: f64, aspect_ratio:f64) -> Camera{
        let theta = deg_to_rad(v_fov);
        let h = (0.5*theta).tan();
        let viewport_height = 2.0*h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vector();
        let u = Vec3::cross(&v_up, &w);
        let v = Vec3::cross(&w, &u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - w;
        Camera{origin, horizontal, vertical, lower_left_corner}
    }

    pub fn get_ray(&self, s: f64, t:f64) -> Ray{
        Ray::new(self.origin, self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin)
    }
}

