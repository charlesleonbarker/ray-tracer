use crate::vec::*;

use std::f64::INFINITY;
use std::f64::consts::PI;

pub fn deg_to_rad(deg:f64) -> f64{
    deg*PI/180.0
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