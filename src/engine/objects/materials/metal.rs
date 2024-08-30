use crate::engine::base::ray::Ray;
use crate::engine::base::vector::Vector3;
use crate::engine::objects::materials::material::Material;
use crate::engine::objects::object::HitRecord;
use crate::util::color::Color;

pub struct Metal{
    albedo: Color,
    fuzz : f32
}

impl Metal {
    pub fn new(albedo: Color, fuzz : f32) -> Metal {
        if fuzz < 1.0 {
            Metal {
                albedo,
                fuzz
            }
        } else {
            Metal {
                albedo,
                fuzz: 1.0
            }
        }
    }
}

impl Material for Metal{
    fn scatter(&self, ray_in : &Ray, scattered_ray: &mut Ray, hit_record: &HitRecord, attenuation: &mut Color) -> bool {
        let mut reflected = hit_record.normal.reflect(&scattered_ray.direction);
        reflected = reflected.unit_vector() + (self.fuzz * Vector3::random_unit_vector());

        *scattered_ray = Ray::new(hit_record.point, reflected);

        attenuation.r = self.albedo.r;
        attenuation.g = self.albedo.g;
        attenuation.b = self.albedo.b;

        return scattered_ray.direction.dot(&hit_record.normal) > 0.0;
    }

    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(Metal::new(self.albedo.clone(), self.fuzz))
    }
}

