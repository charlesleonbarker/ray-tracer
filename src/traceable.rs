extern crate fastrand;

use crate::vec::*;
use crate::ray::*;
use crate::bvh::*;
use crate::material::*;
use crate::triangle::*;
use std::clone;
use std::ops::Index;
use core::cmp::Ordering;
use std::convert::TryFrom;

#[derive (Copy, Clone)]
pub struct HitRecord<'a>{
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: &'a dyn Scatter,
    pub p_err: Vec3,
}

#[derive (Default)]
pub struct TraceableList{
    list: Vec<Box<dyn Traceable>>
}

pub enum TraceResult{
    Missed,
    Absorbed(Color),
    Scattered((Color, Ray))
}


pub trait Traceable: Hit{
    fn box_clone(&self) -> Box<dyn Traceable>;
    fn trace(&self, r: &Ray, t_min: f64, t_max: f64) -> TraceResult;
}
 impl<T: Hit> Traceable for T where T: Clone + 'static{
     fn box_clone(&self) -> Box<dyn Traceable>{
         Box::new((*self).clone())
     }
     fn trace(&self, r: &Ray, t_min: f64, t_max: f64) -> TraceResult{
        if let Some(hit_rec) = self.hit(r, t_min, t_max) {
            if let Some((attenuation, scattered)) = hit_rec.mat.scatter(r, &hit_rec){
                TraceResult::Scattered((hit_rec.mat.emit() + attenuation, scattered))
            } else{
                TraceResult::Absorbed(hit_rec.mat.emit())
            }
        } else{
            TraceResult::Missed
        }
    }
 }


impl<'a> HitRecord<'a>{
    pub fn new(p: Point3, normal: Vec3, t: f64, r: Ray, mat: &'a dyn Scatter, p_err: Vec3) -> HitRecord<'a>{
        let mut rec = HitRecord{p, normal, t, front_face: true, mat, p_err};
        rec.set_face_normal(&r, &normal);
        rec      
    }

    pub fn from_mat(mat: &'a dyn Scatter) -> HitRecord<'a>{
        HitRecord::new(Point3::default(), Vec3::default(), 0.0, Ray::default(), mat, Vec3::default())
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

impl TraceableList{

    pub fn new() -> TraceableList{
        TraceableList{list: Vec::new()}
    } 
    

    pub fn add(&mut self, new_traceable: Box<dyn Traceable>)
    {
        self.list.push(new_traceable);
    }

    pub fn remove(&mut self, index: usize) -> Box<dyn Traceable>{
        self.list.remove(index)
    }

    pub fn get(&self, index: usize) -> &dyn Traceable{
        &*self.list[index]
    }

    pub fn len(&self) -> usize{
        self.list.len()
    }

    pub fn empty(&self) -> bool{
        self.list.len() == 0
    }

    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&Box<dyn Traceable>, &Box<dyn Traceable>) -> Ordering,
    {
        self.list.sort_by(compare);
    }

    pub fn split_off(&mut self, at: usize) -> TraceableList{
        TraceableList{list: self.list.split_off(at)}
    }

    pub fn to_Bvh(self) -> BvhNode{
        BvhNode::new(self)
    }

    pub fn add_obj(&mut self, models: Vec<tobj::Model>, mut materials_opt: Option<Vec<tobj::Material>>){
        for  m in models.iter(){
           //if m.name == "wheel_fr_Circle.050_MAIN"{
                let mesh = &m.mesh;
                let pos = &mesh.positions;
                let norms = &mesh.normals;
                let model_color:Color;
                match &materials_opt{
                    Some(mat) =>{
                        let mat_id = mesh.material_id.unwrap();
                        model_color = Color::new(mat[mat_id].diffuse[0] as f64, mat[mat_id].diffuse[1] as f64, mat[mat_id].diffuse[2] as f64);
                    }
                    None =>{
                        model_color = Vec3::new(0.5, 0.5, 0.5);
                    }
                }
                for face_indices in mesh.indices.chunks(3){
                    let mut tri_vert = [Point3::default();3];
                    let mut tri_norm = [Vec3::default(); 3];
                    for vertex in 0..3{
                        tri_vert[vertex] = Point3::new(pos[usize::try_from(face_indices[vertex]*3    ).unwrap()].into(),
                                                    pos[usize::try_from(face_indices[vertex]*3 + 1).unwrap()].into(),
                                                    pos[usize::try_from(face_indices[vertex]*3 + 2).unwrap()].into());
                        tri_norm[vertex] = Vec3::new(norms[usize::try_from(face_indices[vertex]*3    ).unwrap()].into(),
                                                    norms[usize::try_from(face_indices[vertex]*3 + 1).unwrap()].into(),
                                                    norms[usize::try_from(face_indices[vertex]*3 + 2).unwrap()].into());
                    }

                    let tri = Triangle::new(tri_vert, tri_norm, Lambertian::new(model_color));
                    self.add(Box::new(tri));
                }
            //}
        }
    }
}

impl Hit for TraceableList{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>{
        
        let mut closest_so_far = t_max;
        let mut rec_out = None;

        for traceable in &self.list{
            if let Some(rec_temp) = traceable.hit(r, t_min, closest_so_far){
                rec_out = Some(rec_temp);
                closest_so_far = rec_temp.t;
            }
        }
        rec_out
    }

    fn bounding_box(&self) -> Option<Aabb>{
        if self.empty(){
           None
        }else{
            let mut output_box = Aabb::default();
            let mut first_box = true;
            for traceable in &self.list{
                match (traceable.bounding_box(), first_box){
                    (None,_) => {return None}
                    (Some(temp_box),true) => {
                        output_box = temp_box;
                        first_box = false;
                    }
                    (Some(temp_box), false) => {output_box = Aabb::surrounding_box(output_box, temp_box);}
                }
            }
            Some(output_box) 
        }
    }
}

impl Clone for Box<dyn Traceable>{
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

impl Clone for TraceableList{
    fn clone(&self) -> Self {
        TraceableList{list: self.list.clone()}
    }
}


pub trait Hit{
    fn hit(&self ,r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<Aabb>;
}


#[cfg(test)]
mod tests {
    use crate::sphere::*;
    use crate::material::*;
    use super::*;

    #[test]
     fn test_add(){
        let mut list = TraceableList::new();
        let center = Vec3::new(0.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Lambertian::default();
        let s = Box::new(Sphere::new(center, radius, mat));
        list.add(s);
        assert_eq!(list.len(), 1);
     }

    #[test]
    fn test_remove(){
        let mut list = TraceableList::new();
        let center = Vec3::new(0.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Lambertian::default();
        let s = Box::new(Sphere::new(center, radius, mat));
        list.add(s);
        list.remove(0);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_clone(){
        let mut list = TraceableList::new();
        let center = Vec3::new(0.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Lambertian::default();
        let s = Box::new(Sphere::new(center, radius, mat));
        list.add(s);

        let list_clone = list.clone();
        assert_eq!(list_clone.len(), 1);
    }

    #[test]
    fn test_hit(){
        let mut list = TraceableList::new();
        let r = Ray::new(Vec3::new(-10.0, 0.0, 0.0), Vec3::new( 1.0, 0.0, 0.0));
        let t_min = 0.0;
        let t_max = 100.0;

        //Case 1: No intersections
        let center = Vec3::new(0.0, -10.0, 0.0);
        let radius = 5.0;
        let mat = Lambertian::default();
        let s = Box::new(Sphere::new(center, radius, mat));
        list.add(s);
        let hit = list.hit(&r, t_min, t_max);
        assert!(hit.is_none());

        //Case 2: One intersection
        let center = Vec3::new(0.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Lambertian::default();
        let s = Box::new(Sphere::new(center, radius, mat));
        list.add(s);
        let hit = list.hit(&r, t_min, t_max);
        assert!(hit.is_some());
        let rec = hit.unwrap();
        assert_eq!(rec.t, 5.0);  
        
        //Case 3: Two intersections
        let center = Vec3::new(-2.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Lambertian::default();
        let s = Box::new(Sphere::new(center, radius, mat));
        list.add(s);
        let hit = list.hit(&r, t_min, t_max);
        assert!(hit.is_some());
        let rec = hit.unwrap();
        assert_eq!(rec.t, 3.0); 
    }

    #[test]
    fn test_sort_by(){
        let mut list = TraceableList::new();
        let t_min = 0.0;
        let t_max = 100.0;
        for i in 0..101{
            let center = Vec3::new(500.0 - 5.0*(i as f64), 0.0, 0.0);
            let radius = 1.0;
            let mat = Lambertian::default();
            let s = Box::new(Sphere::new(center, radius, mat));
            list.add(s);
        }
        let r = Ray::new(Vec3::new(0.0, -10.0, 0.0), Vec3::new( 0.0, 1.0, 0.0));
        let hit = list.hit(&r, t_min, t_max).unwrap();
        assert_eq!(hit.t, 9.0);
        list.sort_by(|a, b| Aabb::box_compare(a, b, 0));
        list.remove(0);
        let hit = list.hit(&r, t_min, t_max);
        assert!(hit.is_none());
    }

    // #[test]
    // fn add_obj(){
    //     let mut mesh_1 = tobj::Mesh::default();
    //     mesh_1.positions = vec!(-2.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 2.0, 0.0,
    //                             -2.0, 0.0, 5.0, 2.0, 0.0, 5.0, 0.0, 2.0, 5.0,
    //                             -2.0, 0.0, 8.0, 2.0, 0.0, 8.0, 0.0, 2.0, 8.0);

    //     mesh_2.positions = vec!(-2.0, 0.0, 5.0, 2.0, 0.0, 5.0, 0.0, 2.0, 5.0);

    //     let test_1 = tobj::Model::new(mesh_1, "test_1".to_string());

    //     let mesh_2 = tobj::Mesh::default();
    //     let test_2 = tobj::Model::new(mesh_2, "test_2".to_string());

    //     let mesh_3 = tobj::Mesh::default();
    //     let test_3 = tobj::Model::new(mesh_3, "test_3".to_string());

        

    // }
}
