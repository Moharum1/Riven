use crate::engine::base::ray::Ray;
use crate::engine::base::vector::Vector3;
use crate::engine::lighting::diffuse_lighting_model::AnyMaterial;
use crate::engine::lighting::diffuse_lighting_model::material::DiffuseMaterial;
use crate::engine::objects::object::HitRecord;
use crate::util::color::Color;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian {
            albedo
        }
    }
}

impl DiffuseMaterial for Lambertian {
    fn scatter(&self, ray_in: &Ray, scattered_ray: &mut Ray, hit_record: &HitRecord, attenuation: &mut Color) -> bool {
        let mut scatter_direction = hit_record.normal + Vector3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        *scattered_ray = Ray::new(hit_record.point, scatter_direction);

        attenuation.r = self.albedo.r;
        attenuation.g = self.albedo.g;
        attenuation.b = self.albedo.b;

        return true;
    }

    fn clone_box(&self) -> AnyMaterial {
        Box::new(Lambertian::new(self.albedo.clone()))
    }
}