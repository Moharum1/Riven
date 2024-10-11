use crate::engine::base::point::Point3;
use crate::engine::base::ray::Ray;
use crate::engine::base::vector::Vector3;
use crate::engine::lighting::diffuse_lighting_model::MaterialType;
use crate::engine::lighting::diffuse_lighting_model::material::DiffuseMaterial;
use crate::engine::lighting::diffuse_lighting_model::HitRecord;
use crate::engine::textures::solid_color::SolidColor;
use crate::engine::textures::TextureType;
use crate::engine::textures::Texture;
use crate::util::color::Color;

#[derive(Clone, Default)]
pub struct Lambertian {
    albedo: TextureType,
}

impl Lambertian {
    pub fn new(r:f32, g:f32, b: f32) -> MaterialType {
        MaterialType::Lambertian(Lambertian {
            albedo: SolidColor::from_rgb(r, g, b)
        })
    }

    pub fn from_texture(texture : TextureType) -> MaterialType {
        MaterialType::Lambertian(Lambertian {
            albedo: texture
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


        let texture_color = self.albedo.value(hit_record.u, hit_record.v, hit_record.point);

        attenuation.r = texture_color.r;
        attenuation.g = texture_color.g;
        attenuation.b = texture_color.b;

        return true;
    }

    fn clone_box(&self) -> MaterialType {
        Lambertian::new(
            self.albedo.value(0.0, 0.0, Point3::default()).r,
            self.albedo.value(0.0, 0.0, Point3::default()).g,
            self.albedo.value(0.0, 0.0, Point3::default()).b
        )
    }
}