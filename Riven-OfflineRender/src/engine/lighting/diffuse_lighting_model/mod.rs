use crate::engine::base::ray::Ray;
use crate::engine::lighting::diffuse_lighting_model::dielectric::Dielectric;
use crate::engine::lighting::diffuse_lighting_model::lambertian::Lambertian;
use crate::engine::lighting::diffuse_lighting_model::material::DiffuseMaterial;
use crate::engine::lighting::diffuse_lighting_model::metal::Metal;
use crate::engine::objects::hit_record::HitRecord;
use crate::util::color::Color;

pub mod material;
pub mod lambertian;
pub mod metal;
pub mod dielectric;


#[derive(Clone)]
pub enum MaterialType {

    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}


impl MaterialType{
    pub fn scatter(&self, ray_in: &Ray, scattered_ray: &mut Ray, hit_record: &HitRecord, attenuation: &mut Color) -> bool {
        match self {
            MaterialType::Lambertian(lambertian) => lambertian.scatter(ray_in, scattered_ray, hit_record, attenuation),
            MaterialType::Metal(metal) => metal.scatter(ray_in, scattered_ray, hit_record, attenuation),
            MaterialType::Dielectric(dielectric) => dielectric.scatter(ray_in, scattered_ray, hit_record, attenuation),
        }
    }

}

impl Default for MaterialType {
    fn default() -> Self {
        MaterialType::Lambertian(Lambertian::default())
    }
}

pub type AnyMaterial = Box<dyn DiffuseMaterial>;