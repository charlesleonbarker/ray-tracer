use crate::vec::*;
use crate::ray::*;

#[derive (Copy, Clone, Default)]
pub struct HitRecord{
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool
}

// pub struct HittablesList<'a>(Vec<&'a dyn Hittable>);
#[derive (Default)]
pub struct HittablesList<'a>{
    list: Vec<&'a dyn Hittable>
}

impl HitRecord{
    pub fn new(p: &Point3, normal: &Vec3, t: f64, r: &Ray, outward_normal: &Vec3) -> HitRecord{
        let mut rec = HitRecord{p: *p, normal: *normal, t, front_face: true};
        rec.set_face_normal(r, outward_normal);
        rec        
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3){
        self.front_face = Vec3::dot(&r.direction(), outward_normal) <= 0.0;
        if self.front_face{
            self.normal = *outward_normal;
        } else{
            self.normal = -*outward_normal;
        }
    }

    pub fn p(&self) -> Vec3{
        self.p
    }

    pub fn normal(&self) -> Vec3{
        self.normal
    }

    pub fn t(&self) -> f64{
        self.t
    }

    pub fn front_face(&self) -> bool{
        self.front_face
    }
}

impl<'a> HittablesList<'a> {

    pub fn new() -> HittablesList<'a>{
        HittablesList{list: Vec::new()}
    } 

    pub fn add<T>(&mut self, new_hittable: &'a T)
    where T:Hittable + PartialEq + 'a
    {
        self.list.push(new_hittable);
    }

    pub fn get(&mut self, index: usize) -> &'a dyn Hittable{
        self.list[index]
    }

    pub fn remove(&mut self, index: usize){
        self.list.remove(index);
    }

    pub fn len(&self) -> usize{
        self.list.len()
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool{
        
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for &hittable in &self.list{
            if hittable.hit(r, t_min, closest_so_far, &mut temp_rec){
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        hit_anything
    }
}

pub trait Hittable{
    fn hit(&self ,r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::sphere::*;
    use super::*;

    #[test]
     fn test_add(){
        let mut list = HittablesList::new();
        let center = Vec3::new(0.0, 0.0, 0.0);
        let radius = 5.0;
        let s = Sphere::new(&center, radius);
        list.add(&s);
        assert_eq!(list.len(), 1);
     }

    #[test]
    fn test_remove(){
        let mut list = HittablesList::new();
        let center = Vec3::new(0.0, 0.0, 0.0);
        let radius = 5.0;
        let s = Sphere::new(&center, radius);
        list.add(&s);
        list.remove(0);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_hit(){
        //Case 1: No intersections
        let mut list = HittablesList::new();
        let center = Vec3::new(0.0, -10.0, 0.0);
        let radius = 5.0;
        let s = Sphere::new(&center, radius);
        list.add(&s);
        let r = Ray::new(Vec3::new(-10.0, 0.0, 0.0), Vec3::new( 1.0, 0.0, 0.0));
        let t_min = 0.0;
        let t_max = 100.0;
        let mut rec = HitRecord::default();
        let hit = list.hit(&r, t_min, t_max, &mut rec);
        assert_eq!(hit, false);

        //Case 2: One intersection
        let center = Vec3::new(0.0, 0.0, 0.0);
        let radius = 5.0;
        let s = Sphere::new(&center, radius);
        list.add(&s);
        let hit = list.hit(&r, t_min, t_max, &mut rec);
        assert_eq!(hit, true);
        assert_eq!(rec.t, 5.0);  
        
        //Case 3: Two intersections
        let center = Vec3::new(-2.0, 0.0, 0.0);
        let radius = 5.0;
        let s = Sphere::new(&center, radius);
        list.add(&s);
        let hit = list.hit(&r, t_min, t_max, &mut rec);
        assert_eq!(hit, true);
        assert_eq!(rec.t, 3.0);
    }
}
