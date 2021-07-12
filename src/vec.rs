use std::{fs::File, ops};

#[derive (PartialEq, Debug, Copy, Clone)]
pub struct Vec3{
    x: f64,
    y: f64,
    z: f64,
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3{
    pub fn new(x: f64, y: f64, z:f64) -> Vec3{
        Vec3{x,y,z}
    }

    pub fn x(&self) -> f64{
        self.x
    }

    pub fn y(&self) -> f64{
        self.y
    }

    pub fn z(&self) -> f64{
        self.z
    }

    pub fn length(self) -> f64{
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64{
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn dot(lhs:Vec3, rhs:Vec3) -> f64{
        lhs.x*rhs.x + lhs.y*rhs.y + lhs.z*rhs.z
    }

    pub fn unit_vector(self) -> Vec3{
        self/(self.length())
    }

    pub fn cross(lhs: Vec3, rhs: Vec3) -> Vec3{
        Vec3::new(lhs.y*rhs.z - lhs.z*rhs.y, 
                  lhs.z*rhs.x - lhs.x*rhs.z,
                  lhs.x*rhs.y - lhs.y*rhs.x)
    }

}

//Operator overloading using impl_ops
impl_op_ex_commutative!(+ |lhs: &f64, rhs: &Vec3| -> Vec3 { Vec3::new(rhs.x + lhs, rhs.y + lhs, rhs.z + lhs)});
impl_op_ex!(+ |lhs: &Vec3, rhs: &Vec3| -> Vec3 { Vec3::new(lhs.x + rhs.x, lhs.y + rhs.y, lhs.z + rhs.z)});

impl_op_ex!(- |lhs: &f64, rhs: &Vec3| -> Vec3 { Vec3::new(lhs - rhs.x, lhs - rhs.y, lhs - rhs.z)});
impl_op_ex!(- |lhs: &Vec3, rhs: &f64| -> Vec3 { Vec3::new(lhs.x - rhs, lhs.y - rhs, lhs.z - rhs)});
impl_op_ex!(- |lhs: &Vec3, rhs: &Vec3| -> Vec3 { Vec3::new(lhs.x - rhs.x, lhs.y - rhs.y, lhs.z - rhs.z)});

impl_op_ex_commutative!(* |lhs: &f64, rhs: &Vec3| -> Vec3 { Vec3::new(rhs.x * lhs, rhs.y * lhs, rhs.z * lhs)});
impl_op_ex!(/ |lhs: &Vec3, rhs: &f64| -> Vec3 { Vec3::new(lhs.x / rhs, lhs.y / rhs, lhs.z / rhs)});

impl ops::Neg for Vec3{
    type Output = Vec3;
    fn neg(self) -> Vec3{
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Neg for &Vec3{
    type Output = Vec3;
    fn neg(self) -> Vec3{
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Color{

    pub fn write_color<T: std::io::Write>(self, writer: &mut T)
    {
        let ir = (255.999*self.x) as i64;
        let ig = (255.999*self.y) as i64;
        let ib = (255.999*self.z) as i64;
        write!(writer, "{} {} {}\n", ir, ig, ib);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new(){
        let result = Vec3{x: 1.0, y: 2.0, z: 3.0};
        assert_eq!(Vec3::new(1.0,2.0,3.0), result);
    }

    #[test]
    fn test_x(){
        let vec = Vec3{x: 1.0, y: 2.0, z: 3.0};
        assert_eq!(vec.x, 1.0);
    }

    #[test]
    fn test_y(){
        let vec = Vec3{x: 1.0, y: 2.0, z: 3.0};
        assert_eq!(vec.y, 2.0);
    }

    #[test]
    fn test_z(){
        let vec = Vec3{x: 1.0, y: 2.0, z: 3.0};
        assert_eq!(vec.z, 3.0);
    }

    #[test]
    fn test_vec_add(){
        let lhs = Vec3::new(1.0,2.0,3.0);
        let rhs = Vec3::new(4.0,2.0,-1.0);
        let result = Vec3::new(5.0,4.0,2.0);
        assert_eq!(lhs + rhs, result);
    }

    #[test]
    fn test_scalar_add(){
        let lhs = Vec3::new(1.0,2.0,3.0);
        let rhs = 2.0;
        let result = Vec3::new(3.0,4.0,5.0);
        assert_eq!(lhs + rhs, result);
    }

    #[test]
    fn test_vec_sub(){
        let lhs = Vec3::new(1.0,2.0,3.0);
        let rhs = Vec3::new(4.0,2.0,-1.0);
        let result = Vec3::new(-3.0,0.0,4.0);
        assert_eq!(lhs - rhs, result);
    }

    #[test]
    fn test_scalar_sub(){
        let lhs = Vec3::new(1.0,2.0,3.0);
        let rhs = 2.0;
        let result = Vec3::new(-1.0,0.0,1.0);
        assert_eq!(lhs - rhs, result);
    }

    #[test]
    fn test_scalar_mul(){
        let lhs = Vec3::new(1.0,2.0,3.0);
        let rhs = 5.0;
        let result = Vec3::new(5.0,10.0,15.0);
        assert_eq!(lhs * rhs, result);
    }

    #[test]
    fn test_scalar_div(){
        let lhs = Vec3::new(1.0,2.0,3.0);
        let rhs = 5.0;
        let result = Vec3::new(0.2,0.4,0.6);
        assert_eq!(lhs / rhs, result);
    }

    #[test]
    fn test_dot(){
        let lhs = Vec3::new(1.0,2.0,3.0);
        let rhs = Vec3::new(4.0,2.0,-1.0);
        let result = 5.0;
        assert_eq!(Vec3::dot(lhs,rhs), result);
    }

    #[test]
    fn test_length(){
        let lhs = Vec3::new(4.0,3.0,0.0);
        let result = 5.0;
        assert_eq!(lhs.length(), result);
    }

    #[test]
    fn test_length_squared(){
        let lhs = Vec3::new(1.0,2.0,3.0);
        let result = 14.0;
        assert_eq!(lhs.length_squared(), result);
    }

    #[test]
    fn test_unit_vector(){
        let vec = Vec3::new(2.0,0.0,0.0);
        let result = Vec3::new(1.0,0.0,0.0);
        assert_eq!(vec.unit_vector(), result);
    }

    #[test]
    fn test_cross(){
        let lhs = Vec3::new(3.0,-3.0,1.0);
        let rhs = Vec3::new(4.0,9.0,2.0);
        let result = Vec3::new(-15.0, -2.0, 39.0);
        assert_eq!(Vec3::cross(lhs, rhs), result);
    }

    #[test]
    fn test_neg(){
        let vec = Vec3::new(3.0,-3.0,1.0);
        let result = Vec3::new(-3.0, 3.0, -1.0);
        assert_eq!(-vec, result);
    }

}
