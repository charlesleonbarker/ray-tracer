use crate::vec::*;
use crate::ray::*;
use crate::hittable::*;

#[derive (PartialEq, Copy, Clone, Default)]
pub struct Sphere{
    center: Point3,
    radius: f64,
}

impl Sphere{
    pub fn new(cen: &Point3, rad: f64) -> Sphere{
        Sphere{center: *cen, radius: rad}
    }
}

impl Hittable for Sphere{
    fn hit(&self, r:&Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool{
        let oc = &r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius*self.radius;
        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0{
            false
        }else{
            let sqrtd = discriminant.sqrt();
            let mut root = (-half_b - sqrtd)/a;
            if root < t_min || t_max < root{
                root = (-half_b + sqrtd)/a;
                if root < t_min || t_max < root{
                    return false
                }
            }
            rec.t = root;
            rec.p = r.at(rec.t);
            rec.normal = (rec.p - self.center)/self.radius;
            let outward_normal = (rec.p - self.center)/self.radius;
            rec.set_face_normal(r, &outward_normal);
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new(){
        let center = Vec3::new(0.0, 0.0, 0.0);
        let radius = 5.0;
        let s = Sphere::new(&center, radius);
        assert_eq!(s.center, Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(s.radius, 5.0);
    }

    #[test]
    fn test_hit(){

        //Case 1: Intersection from outside of sphere
        let center = Vec3::new(0.0, 0.0, 0.0);
        let radius = 5.0;
        let s = Sphere::new(&center, radius);
        let r = Ray::new(Vec3::new(-10.0, 0.0, 0.0), Vec3::new( 1.0, 0.0, 0.0));
        let t_min = 0.0;
        let t_max = 100.0;
        let mut rec = HitRecord::default();
        let hit = s.hit(&r, t_min, t_max,&mut rec);
        assert_eq!(hit, true);
        assert_eq!(rec.t(), 5.0);
        assert_eq!(rec.normal(), Vec3::new(-1.0, 0.0, 0.0));
        assert_eq!(rec.p(), Point3::new(-5.0, 0.0, 0.0));
        assert_eq!(rec.front_face(), true);

        //Case 2: Intersection from inside of sphere
        let r = Ray::new(Vec3::new(1.0, 0.0, 0.0), Vec3::new( -2.0, 0.0, 0.0));
        let mut rec = HitRecord::default();
        let hit = s.hit(&r, t_min, t_max,&mut rec);
        assert_eq!(hit, true);
        assert_eq!(rec.t(), 3.0);
        assert_eq!(rec.normal(), Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(rec.p(), Point3::new(-5.0, 0.0, 0.0));
        assert_eq!(rec.front_face(), false);

        //Case 3: Intersection tangent to sphere
        let r = Ray::new(Vec3::new(-5.0, 5.0, 0.0), Vec3::new( 0.0, -1.0, 0.0));
        let mut rec = HitRecord::default();
        let hit = s.hit(&r, t_min, t_max,&mut rec);
        assert_eq!(hit, true);
        assert_eq!(rec.t(), 5.0);
        assert_eq!(rec.normal(), Vec3::new(-1.0, 0.0, 0.0));
        assert_eq!(rec.p(), Point3::new(-5.0, 0.0, 0.0));
        assert_eq!(rec.front_face(), true);
    }
}