use crate::vec::*;
use crate::ray::*;
use crate::traceable::*;

#[derive(Default)]
pub struct Lambertian{
    albedo: Color
}

#[derive(Default)]
pub struct Metal{
    albedo: Color
}


impl Lambertian{
    pub fn new(alb: Color) -> Lambertian{
        Lambertian{albedo: alb}
    }
}

impl Metal{
    pub fn new(alb: Color) -> Metal{
        Metal{albedo: alb}
    }
}

impl Scatter for Lambertian{
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>
    {
        let mut scatter_direction = rec.normal + Vec3::rand_unit_vec();

        // Catch degenerate Scatter direction
        if scatter_direction.near_zero(){
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

impl Scatter for Metal{
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>{
        let reflected = r_in.direction().unit_vector().reflect(&rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;
        if Vec3::dot(&scattered.direction(), &rec.normal) > 0.0{
            Some((attenuation, scattered))
        }else{
            None
        }
    }
}
