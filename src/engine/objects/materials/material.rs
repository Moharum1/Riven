use crate::engine::base::ray::Ray;
use crate::engine::objects::object::HitRecord;
use crate::util::color::Color;


pub trait Material{
    fn scatter(&self, ray_in : &Ray, scattered_ray: &mut Ray, hit_record: &HitRecord, attenuation: &mut Color) -> bool;

    fn clone_box(&self) -> Box<dyn Material>;
}

impl Default for Box<dyn Material> {
    fn default() -> Self {
        Box::new(DEFAULT_MAT)
    }
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.clone_box()
    }
}


// Basic material

pub struct BasicMaterial {
    pub albedo: Color,
}

impl Material for BasicMaterial {
    fn scatter(&self, _ray_in: &Ray, _scattered_ray: &mut Ray, _hit_record: &HitRecord, attenuation: &mut Color) -> bool {
        println!("BasicMaterial scatter");
        false  // No scattering, so we return false
    }

    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(DEFAULT_MAT)
    }
}
const RED_COLOR : Color = Color::new(100.0, 1.0, 1.0);
pub const DEFAULT_MAT: BasicMaterial = BasicMaterial {
    albedo: RED_COLOR,
};

