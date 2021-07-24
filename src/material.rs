use crate::vec::*;
use crate::ray::*;
use crate::traceable::*;
use crate::*;


#[derive(Default)]
pub struct Lambertian{
    albedo: Color
}

#[derive(Default)]
pub struct Metal{
    albedo: Color,
    fuzz: f64
}

pub struct Dielectric{
    index_of_refraction :f64,
}


impl Lambertian{
    pub fn new(alb: Color) -> Lambertian{
        Lambertian{albedo: alb}
    }
}

impl Metal{
    pub fn new(alb:Vec3, mut fuzz: f64) -> Metal{
        if fuzz > 1.0 {fuzz = 1.0}
        else if fuzz < 0.0 {fuzz = 0.0}
        Metal{albedo: alb, fuzz}
    }
}

impl Dielectric{
    pub fn new(ir: f64) -> Dielectric{
        Dielectric{index_of_refraction: ir}
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64{
        //Use Schlick's approximation for reflectance
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
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
        let scattered = Ray::new(rec.p, reflected + self.fuzz*Vec3::rand_in_unit_sphere());
        let attenuation = self.albedo;
        if Vec3::dot(&scattered.direction(), &rec.normal) > 0.0{
            Some((attenuation, scattered))
        }else{
            None
        }
    }
}

impl Scatter for Dielectric{
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>{
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let mut refraction_ratio = self.index_of_refraction;
        if rec.front_face == true{
            refraction_ratio = 1.0/self.index_of_refraction;
        }
        
        let unit_dir = r_in.direction().unit_vector();
        let cos_theta = Vec3::dot(&-unit_dir, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract = refraction_ratio*sin_theta > 1.0;
        let direction:Vec3;

        if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > rand_double(0.0, 1.0){
            direction = Vec3::reflect(&unit_dir, &rec.normal);
        } else{
            direction = Vec3::refract(&unit_dir, &rec.normal, refraction_ratio);
        }
        let scattered = Ray::new(rec.p, direction);
        Some((attenuation, scattered))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reflectance(){
        let unit_vec = Vec3::new(1.0, 2.0, 3.0).unit_vector();
        let normal = Vec3::new(2.0, -1.0, 1.0);
        let cos_theta = Vec3::dot(&-unit_vec, &normal).min(1.0);
        let refraction_ratio = 2.0;
        assert_eq!(Dielectric::reflectance(cos_theta,refraction_ratio), 1.0/9.0 + (8.0/9.0)*((1.0-cos_theta).powi(5)));
    }
}