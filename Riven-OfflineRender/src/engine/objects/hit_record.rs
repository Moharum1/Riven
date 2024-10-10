use crate::engine::base::point::Point3;
use crate::engine::base::ray::Ray;
use crate::engine::base::vector::Vector3;
use crate::engine::lighting::diffuse_lighting_model::MaterialType;

/// A struct representing a record of a hit in ray tracing.
#[derive(Default)]
pub struct HitRecord {
    /// The point at which the hit occurred.
    pub point: Point3,
    /// The normal vector at the hit point.
    pub normal: Vector3,
    /// The parameter `t` at which the hit occurred.
    pub t: f32,
    /// A boolean indicating whether the hit was on the front face.
    pub front_face: bool,

    pub mat: MaterialType,
}


impl HitRecord {
    /// Sets the face normal of the hit record based on the ray and outward normal.
    ///
    /// # Arguments
    ///
    /// * `ray` - The ray that hit the object.
    /// * `outward_normal` - The normal vector pointing outward from the hit point.
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3) {
        self.front_face = ray.direction.dot(&outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}