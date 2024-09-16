use crate::engine::base::constants::constants::random_float;
use crate::engine::base::ray::Ray;
use crate::engine::lighting::diffuse_lighting_model::AnyMaterial;
use crate::engine::lighting::diffuse_lighting_model::material::DiffuseMaterial;
use crate::engine::objects::object::HitRecord;
use crate::util::color::Color;

pub struct Dielectric {
    refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Dielectric {
        Dielectric {
            refraction_index
        }
    }

    fn reflectance(&self, cosine: f32, refraction_index: f32) -> f32 {
        let mut r0 = (1f32 - refraction_index) / (1f32 + refraction_index);
        r0 = r0 * r0;
        return r0 + (1f32 - r0) * (1f32 - cosine).powi(5);
    }
}

impl DiffuseMaterial for Dielectric {
    fn scatter(&self, ray_in: &Ray, scattered_ray: &mut Ray, hit_record: &HitRecord, attenuation: &mut Color) -> bool {
        attenuation.r = 1.0;
        attenuation.g = 1.0;
        attenuation.b = 1.0;

        let ri = if hit_record.front_face { 1.0 / self.refraction_index } else { self.refraction_index };
        let unit_direction = ray_in.direction.unit_vector();

        let cos_theta = -unit_direction.dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        if ri * sin_theta > 1.0 || self.reflectance(cos_theta, ri) > random_float() {
            let reflected = unit_direction.reflect(&hit_record.normal);
            *scattered_ray = Ray::new(hit_record.point, reflected);
        } else {
            let refracted = unit_direction.refract(&hit_record.normal, ri);
            *scattered_ray = Ray::new(hit_record.point, refracted);
        }

        true
    }

    fn clone_box(&self) -> AnyMaterial {
        Box::new(Dielectric::new(self.refraction_index))
    }
}

