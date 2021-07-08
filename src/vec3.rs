use std::ops;
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
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64{
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn dot(lhs:vec3, rhs:vec3) -> f64{
        lhs.x*rhs.x + lhs.y*rhs.y + lhs.z*rhs.z
    }

    pub fn unit_vector(self) -> vec3{
        self/(self.length())
    }

}

impl<T> ops::Add<T> for vec3 where T:Vec3_Add {
    type Output = vec3;
    fn add(self, rhs: T) -> vec3 {
        rhs.vec3_add(self)
    }
}

impl<T> ops::Sub<T> for vec3 where T:Vec3_Sub {
    type Output = vec3;
    fn sub(self, rhs: T) -> vec3 {
        rhs.vec3_sub(self)
    }
}

impl<T> ops::Mul<T> for vec3 where T:Vec3_Mul {
    type Output = vec3;
    fn mul(self, rhs: T) -> vec3 {
        rhs.vec3_mul(self)
    }
}

impl<T> ops::Div<T> for vec3 where T:Vec3_Div {
    type Output = vec3;
    fn div(self, rhs: T) -> vec3 {
        rhs.vec3_div(self)
    }
}
pub trait Vec3_Add{
    fn vec3_add(self, vec: vec3) -> vec3;
}

pub trait Vec3_Sub{
    fn vec3_sub(self, vec: vec3) -> vec3;
}

pub trait Vec3_Mul{
    fn vec3_mul(self, vec: vec3) -> vec3;
}

pub trait Vec3_Div{
    fn vec3_div(self, vec: vec3) -> vec3;
}

impl Vec3_Add for f64{
    fn vec3_add(self, vec:vec3) -> vec3{
        vec3::new(vec.x + self, vec.y + self, vec.z + self)
    }
}

impl Vec3_Sub for f64{
    fn vec3_sub(self, vec:vec3) -> vec3{
        vec3::new(vec.x - self, vec.y - self, vec.z - self)
    }
}

impl Vec3_Mul for f64{
    fn vec3_mul(self, vec:vec3) -> vec3{
        vec3::new(vec.x*self, vec.y*self, vec.z*self)
    }
}

impl Vec3_Div for f64{
    fn vec3_div(self, vec:vec3) -> vec3{
        vec3::new(vec.x/self, vec.y/self, vec.z/self)
    }
}

impl Vec3_Add for vec3{
    fn vec3_add(self, other:vec3) -> vec3{
        vec3::new(other.x + self.x, other.y + self.y, other.z + self.z)
    }
}

impl Vec3_Sub for vec3{
    fn vec3_sub(self, other:vec3) -> vec3{
        vec3::new(other.x - self.x, other.y - self.y, other.z - self.z)
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
        assert_eq!(lhs * rhs, result);
    }

    #[test]
    fn test_div(){
        let lhs = vec3::new(1.0,2.0,3.0);
        let rhs = 5.0;
        let result = vec3::new(5.0,10.0,15.0);
        assert_eq!(lhs * rhs, result);
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
        let lhs = vec3::new(4.0,3.0,0.0);
        let result = 5.0;
        assert_eq!(lhs.length(), result);
    }

    #[test]
    fn test_length_squared(){
        let lhs = vec3::new(1.0,2.0,3.0);
        let result = 14.0;
        assert_eq!(lhs.length_squared(), result);
    }

    #[test]
    fn test_unit_vector(){
        let vec = vec3::new(2.0,0.0,0.0);
        let result = vec3::new(1.0,0.0,0.0);
        assert_eq!(vec.unit_vector(), result);
    }

}
