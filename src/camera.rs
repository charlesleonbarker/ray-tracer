use core::f64;

use crate::deg_to_rad;
use crate::vec::*;
use crate::ray::*;

pub const ASPECT_RATIO: f64 = 16.0/9.0;
pub const VIEWPORT_HEIGHT: f64 = 2.0;
pub const VIEWPORT_WIDTH: f64 = ASPECT_RATIO*VIEWPORT_HEIGHT;
pub const FOCAL_LENGTH: f64 = 1.0;

#[derive (Copy, Clone, Default)]
pub struct Camera{
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3
}

impl Camera{

    pub fn new(vfov: f64, aspect_ratio:f64) -> Camera{
        let theta = deg_to_rad(vfov);
        let h = (0.5*theta).tan();
        let viewport_height = 2.0*h;
        let viewport_width = aspect_ratio * viewport_height;

        let origin = Point3::new(0.0,0.0,0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0,0.0, FOCAL_LENGTH);
        Camera{origin, horizontal, vertical, lower_left_corner}
    }

    pub fn get_ray(&self, u: f64, v:f64) -> Ray{
        Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin)
    }
}

