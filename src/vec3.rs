use std::ops;
use num::pow;
#[derive (PartialEq, Debug, Copy, Clone)]
pub struct vec3{
    x: f64,
    y: f64,
    z: f64,
}

type point3 = vec3;
type color = vec3;

impl vec3{
    pub fn new(x: f64, y: f64, z:f64) -> vec3{
        vec3{x,y,z}
    }

    pub fn length(self) -> f64{
        num::pow(self.length_squared(), 1/2)
    }

    pub fn length_squared(self) -> f64{
        num::pow(self.x, 2) + pow(self.y, 2) + pow(self.z, 2)
    }

    fn mul(self, rhs: f64) -> vec3 {
        vec3::new(self.x*rhs, self.y*rhs, self.z*rhs)
    }

    fn div(self, rhs: f64) -> vec3{
        vec3::new(self.x/rhs, self.y/rhs, self.z/rhs)
    }
    pub fn dot(lhs:vec3, rhs:vec3) -> f64{
        lhs.x*rhs.x + lhs.y*rhs.y + lhs.z*rhs.z
    }

}

impl ops::Add for vec3 {
    type Output = vec3;
    fn add(self, rhs: vec3) -> vec3 {
        vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub for vec3 {
    type Output = vec3;
    fn sub(self, rhs: vec3) -> vec3 {
        vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new(){
        let result = vec3{x: 1.0, y: 2.0, z: 3.0};
        assert_eq!(vec3::new(1.0,2.0,3.0), result);
    }

    #[test]
    fn test_add(){
        let lhs = vec3::new(1.0,2.0,3.0);
        let rhs = vec3::new(4.0,2.0,-1.0);
        let result = vec3::new(5.0,4.0,2.0);
        assert_eq!(lhs + rhs, result);
    }

    #[test]
    fn test_sub(){
        let lhs = vec3::new(1.0,2.0,3.0);
        let rhs = vec3::new(4.0,2.0,-1.0);
        let result = vec3::new(-3.0,0.0,4.0);
        assert_eq!(lhs - rhs, result);
    }

    #[test]
    fn test_mul(){
        let lhs = vec3::new(1.0,2.0,3.0);
        let rhs = 5.0;
        let result = vec3::new(5.0,10.0,15.0);
        assert_eq!(lhs.mul(rhs), result);
    }

    #[test]
    fn test_div(){
        let lhs = vec3::new(1.0,2.0,3.0);
        let rhs = 5.0;
        let result = vec3::new(5.0,10.0,15.0);
        assert_eq!(lhs.mul(rhs), result);
    }

    #[test]
    fn test_dot(){
        let lhs = vec3::new(1.0,2.0,3.0);
        let rhs = vec3::new(4.0,2.0,-1.0);
        let result = 5.0;
        assert_eq!(vec3::dot(lhs,rhs), result);
    }

    #[test]
    fn test_length(){
        let lhs = vec3::new(1.0,2.0,3.0);
        let result = pow(14.0, 1/2);
        assert_eq!(lhs.length(), result);
    }

    #[test]
    fn test_length_squared(){
        let lhs = vec3::new(1.0,2.0,3.0);
        let result = 14.0;
        assert_eq!(lhs.length_squared(), result);
    }

}
