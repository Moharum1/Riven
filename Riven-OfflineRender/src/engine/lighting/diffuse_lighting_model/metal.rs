use crate::engine::base::ray::Ray;
use crate::engine::base::vector::Vector3;
use crate::engine::lighting::diffuse_lighting_model::MaterialType;
use crate::engine::lighting::diffuse_lighting_model::material::DiffuseMaterial;
use crate::engine::objects::hit_record::HitRecord;
use crate::util::color::Color;

#[derive(Clone, Default)]
pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(r : f32, g : f32, b : f32, fuzz: f32) -> MaterialType {
        if fuzz < 1.0 {
            MaterialType::Metal(Metal {
                albedo: Color::new(r, g, b),
                fuzz,
            })
        } else {
            MaterialType::Metal(Metal {
                albedo: Color::new(r, g, b),
                fuzz: 1.0,
            })
        }
    }
}

impl DiffuseMaterial for Metal {
    fn scatter(&self, _: &Ray, scattered_ray: &mut Ray, hit_record: &HitRecord, attenuation: &mut Color) -> bool {
        let mut reflected = hit_record.normal.reflect(&scattered_ray.direction);
        reflected = reflected.unit_vector() + (self.fuzz * Vector3::random_unit_vector());

        *scattered_ray = Ray::new(hit_record.point, reflected);

        attenuation.r = self.albedo.r;
        attenuation.g = self.albedo.g;
        attenuation.b = self.albedo.b;

        return scattered_ray.direction.dot(&hit_record.normal) > 0.0;
    }

    fn clone_box(&self) -> MaterialType {
        Metal::new(
            self.albedo.r,
            self.albedo.g,
            self.albedo.b,
            self.fuzz
        )
    }
}

