use crate::vec::*;
use crate::ray::*;
use crate::traceable::*;
use std::cmp::Ordering;

#[derive (Debug, Copy, Clone, Default)]

pub struct Aabb{
    min: Point3,
    max: Point3
}

#[derive(Clone)]
pub struct BvhNode{
    left: Box<dyn Traceable>,
    right: Box<dyn Traceable>,
    bb: Aabb
}

impl Aabb{
    pub fn new(min: Point3, max: Point3) -> Aabb{
        Aabb{min, max}
    }

    pub fn min(&self) -> Point3{
        self.min
    }

    pub fn max(&self) -> Point3{
        self.max
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool{
        for a in 0..3{
            let tx0 = (self.min().index(a) - r.origin().index(a))/r.direction().index(a);
            let tx1 = (self.max().index(a) - r.origin().index(a))/r.direction().index(a);
            
            let t0 = tx0.min(tx1);
            let t1 = tx0.max(tx1);

            let t_min = t0.max(t_min);
            let t_max = t1.min(t_max);

            if t_max <= t_min{
                return false
            }
        }
        true
    }

    pub fn surrounding_box(box_0: Aabb, box_1: Aabb) -> Aabb{
        let small = Point3::new(box_0.min().x().min(box_1.min().x()),
                                     box_0.min().y().min(box_1.min().y()),
                                     box_0.min().z().min(box_1.min().z()));

        let big = Point3::new(box_0.max().x().max(box_1.max().x()),
                                   box_0.max().y().max(box_1.max().y()),
                                   box_0.max().z().max(box_1.max().z()));

        Aabb::new(small, big)
    }

    pub fn box_compare(a: &Box<dyn Traceable>, b: &Box<dyn Traceable>, axis: i8) -> Ordering{
        match (a.bounding_box(), b.bounding_box()){
            (Some(box_a), Some(box_b)) => box_a.min().index(axis as usize).partial_cmp(&box_b.min().index(axis as usize)).unwrap(),
            (_, _) => panic!("One of a, b cannot be bound")
        }
    }
}


impl BvhNode{
    pub fn new(mut objects: TraceableList) -> BvhNode{
        let object_span = objects.len();
        let left: Box<dyn Traceable>;
        let right: Box<dyn Traceable>;
        match object_span {
            1 => {
                //On the slim chance object_span equals 1, simply clone
                //the object. This stops us having to check for None in
                //each child, and makes tree traversal smoother.
                left = objects.remove(0);
                right = left.clone();
            }
            2 => {
                right = objects.remove(1);
                left = objects.remove(0);
            } 
            _ => {
                let axis = fastrand::i8(0..3);
                objects.sort_by(|a, b| Aabb::box_compare(a,b, axis));
                let mid = object_span/2;
                let right_objs = objects.split_off(mid);
                let left_objs = objects;
                left = Box::new(BvhNode::new(left_objs));
                right = Box::new(BvhNode::new(right_objs));
            }
        }

        match(left.bounding_box(), right.bounding_box()){
            (Some(box_left), Some(box_right)) => {
                let bb = Aabb::surrounding_box(box_left, box_right);
                BvhNode{left, right, bb}
            }
            (_, _) => panic!("A primitive within the TraceableList cannot be bound"),

        }
    }
}

impl Hit for BvhNode{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bb.hit(r, t_min, t_max){
            return None
        } 

        let hit_left = self.left.hit(r, t_min, t_max);
        let hit_right = self.right.hit(r, t_min, t_max);
        match(hit_left, hit_right){
            (None, None) => None,
            (Some(_), None) => hit_left,
            (None, Some(_)) => hit_right,
            (Some(left), Some(right)) =>  {
                if left.t <= right.t{
                    hit_left
                } else {
                    hit_right
                }
            }
        }
        
    }
    
    fn bounding_box(&self) -> Option<Aabb> {
        Some(self.bb)
    }
}

#[cfg(test)]
mod tests {
    use crate::sphere::*;
    use crate::material::*;
    use super::*;

    #[test]
    fn min(){
        let min = Point3::new(0.0, -2.0, 1.0);
        let max = Point3::new(10.0, 3.0, 4.0);
        let aabb = Aabb::new(min, max);
        assert_eq!(aabb.min(), Point3::new(0.0, -2.0, 1.0));
    }

    #[test]
    fn max(){
        let min = Point3::new(0.0, -2.0, 1.0);
        let max = Point3::new(10.0, 3.0, 4.0);
        let aabb = Aabb::new(min, max);
        assert_eq!(aabb.max(), Point3::new(10.0, 3.0, 4.0));
    }

    #[test]
    fn test_aabb_hit(){

        let min = Point3::new(0.0, 0.0, 0.0);
        let max = Point3::new(10.0, 10.0, 10.0);
        let aabb = Aabb::new(min, max);

        //Case 1: Hit
        let r = Ray::new(Vec3::new(-10.0, 5.0, 5.0), Vec3::new( 1.0, 0.0, 0.0));
        let rec = aabb.hit(&r, 0.0, 100.0);
        assert_eq!(rec, true);

        //Case 2: Miss (due to timeout)
        let rec = aabb.hit(&r, 0.0, 9.9);
        assert_eq!(rec, false);

        //Case 3: Miss (due to geometry)
        let r = Ray::new(Vec3::new(-10.0, 10.01, 5.0), Vec3::new( 1.0, 0.0, 0.0));
        let rec = aabb.hit(&r, 0.0, 100.0);
        assert_eq!(rec, false);


    }

    #[test]
    fn test_surrounding_box(){
        
        let min = Point3::new(0.0, 0.0, 0.0);
        let max = Point3::new(10.0, 10.0, 10.0);
        let aabb_a = Aabb::new(min, max);

        let min = Point3::new(-1.0, 3.0, -2.0);
        let max = Point3::new(9.0, 16.0, 10.0);
        let aabb_b = Aabb::new(min, max);

        let sb = Aabb::surrounding_box(aabb_a, aabb_b);

        assert_eq!(sb.min(), Point3::new(-1.0, 0.0, -2.0));
        assert_eq!(sb.max(), Point3::new(10.0, 16.0, 10.0));


    }
    #[test]
    fn test_bvhnode_hit(){

        let mut list = TraceableList::new();
        let r = Ray::new(Vec3::new(-10.0, 0.0, 0.0), Vec3::new( 1.0, 0.0, 0.0));
        let t_min = 0.0;
        let t_max = 100.0;

        
        //Case 1: No intersections
        let center = Vec3::new(0.0, -10.0, 0.0);
        let radius = 5.0;
        let mat = Lambertian::default();
        let s = Box::new(Sphere::new(center, radius, mat));
        for _ in 1..100{
            list.add(s.clone());
        }
        let list_clone = list.clone();
        let bvh = list_clone.to_Bvh();
        let hit = bvh.hit(&r, t_min, t_max);
        assert!(hit.is_none());

        //Case 2: Single intersection
        let center = Vec3::new(0.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Lambertian::default();
        let s = Box::new(Sphere::new(center, radius, mat));
        list.add(s);
        let list_clone = list.clone();
        let bvh = list_clone.to_Bvh();
        let hit = bvh.hit(&r, t_min, t_max);
        assert!(hit.is_some());
        let rec = hit.unwrap();
        assert_eq!(rec.t, 5.0); 
        
        //Case 3: Two intersections
        let center = Vec3::new(-2.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Lambertian::default();
        let s = Box::new(Sphere::new(center, radius, mat));
        list.add(s);
        let list_clone = list.clone();
        let bvh = list_clone.to_Bvh();
        let hit = bvh.hit(&r, t_min, t_max);
        assert!(hit.is_some());
        let rec = hit.unwrap();
        assert_eq!(rec.t, 3.0); 
        
    }

    
}