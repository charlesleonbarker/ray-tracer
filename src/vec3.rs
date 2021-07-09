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


impl ops::Add<f64> for vec3{
    type Output = vec3;
    fn add(self, rhs: f64) -> vec3 {
        vec3::new(self.x + rhs, self. y + rhs, self.z + rhs)
    }
}

impl ops::Add<vec3> for f64{
    type Output = vec3;
    fn add(self, rhs: vec3) -> vec3 {
        vec3::new(rhs.x + self, rhs.y + self, rhs.z + self)
    }
}

impl ops::Add<vec3> for vec3{
    type Output = vec3;
    fn add(self, rhs: vec3) -> vec3 {
        vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub<f64> for vec3{
    type Output = vec3;
    fn sub(self, rhs: f64) -> vec3 {
        vec3::new(self.x - rhs, self. y - rhs, self.z - rhs)
    }
}

impl ops::Sub<vec3> for f64{
    type Output = vec3;
    fn sub(self, rhs: vec3) -> vec3 {
        vec3::new(rhs.x - self, rhs.y - self, rhs.z - self)
    }
}

impl ops::Sub<vec3> for vec3{
    type Output = vec3;
    fn sub(self, rhs: vec3) -> vec3 {
        vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Mul<f64> for vec3{
    type Output = vec3;
    fn mul(self, rhs: f64) -> vec3 {
        vec3::new(self.x * rhs, self. y * rhs, self.z * rhs)
    }
}

impl ops::Mul<vec3> for f64{
    type Output = vec3;
    fn mul(self, rhs: vec3) -> vec3 {
        vec3::new(rhs.x * self, rhs.y * self, rhs.z * self)
    }
}

impl ops::Div<f64> for vec3{
    type Output = vec3;
    fn div(self, rhs: f64) -> vec3 {
        vec3::new(self.x / rhs, self. y / rhs, self.z / rhs)
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
        // let rhs = vec3::new(4.0,2.0,-1.0);
        let rhs = 2.0;
        let result = vec3::new(3.0,4.0,5.0);
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
        let result = vec3::new(0.2,0.4,0.6);
        assert_eq!(lhs / rhs, result);
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
