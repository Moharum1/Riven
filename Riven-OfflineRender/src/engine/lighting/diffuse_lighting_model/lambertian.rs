use crate::engine::base::ray::Ray;
use crate::engine::base::vector::Vector3;
use crate::engine::lighting::diffuse_lighting_model::MaterialType;
use crate::engine::lighting::diffuse_lighting_model::material::DiffuseMaterial;
use crate::engine::lighting::diffuse_lighting_model::HitRecord;
use crate::util::color::Color;

#[derive(Clone, Default)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(r:f32, g:f32, b: f32) -> MaterialType {
        MaterialType::Lambertian(Lambertian {
            albedo: Color::new(r, g, b)
        })
    }
}

impl DiffuseMaterial for Lambertian {
    fn scatter(&self, _: &Ray, scattered_ray: &mut Ray, hit_record: &HitRecord, attenuation: &mut Color) -> bool {
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

    fn clone_box(&self) -> MaterialType {
        Lambertian::new(
            self.albedo.r,
            self.albedo.g,
            self.albedo.b
        )
    }
}