use crate::engine::base::ray::Ray;
use crate::engine::lighting::diffuse_lighting_model::MaterialType;
use crate::engine::objects::hit_record::HitRecord;
use crate::util::color::Color;


pub trait DiffuseMaterial : Sync + Send {
    fn scatter(&self, ray_in: &Ray, scattered_ray: &mut Ray, hit_record: &HitRecord, attenuation: &mut Color) -> bool;

    fn clone_box(&self) -> MaterialType;
}

// impl Default for AnyMaterial {
//     fn default() -> Self {
//       Lambertian::new(Color::new(0f32, 0f32, 0f32))
//     }
// }
//
// impl Clone for AnyMaterial {
//     fn clone(&self) -> AnyMaterial {
//         self.clone_box()
//     }
// }