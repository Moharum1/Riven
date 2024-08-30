use std::rc::Rc;
use crate::engine::base::constants::constants;
use crate::engine::base::point::Point3;
use crate::engine::base::vector::Vector3;
use crate::engine::objects::object::{HitList, HitRecord, Object};
use crate::util::color::Color;

/// A struct representing a ray in 3D space.
pub struct Ray {
    /// The origin point of the ray.
    pub(crate) origin: Point3,
    /// The direction vector of the ray.
    pub(crate) direction: Vector3,
}

impl Ray {
    /// Creates a new `Ray` with the specified origin and direction.
    ///
    /// # Arguments
    ///
    /// * `origin` - The origin point of the ray.
    /// * `direction` - The direction vector of the ray.
    ///
    /// # Returns
    ///
    /// A new instance of `Ray`.
    pub fn new(origin: Point3, direction: Vector3) -> Self {
        Self { origin, direction }
    }

    /// Computes the point at a given distance `t` along the ray.
    ///
    /// # Arguments
    ///
    /// * `t` - The distance along the ray.
    ///
    /// # Returns
    ///
    /// A `Point3` representing the point at distance `t` along the ray.
    pub fn at(&self, t: f32) -> Point3 {
        self.origin + self.direction * t
    }

    /// Computes the color of the ray based on its direction.
    ///
    /// # Returns
    ///
    /// A `Color` representing the color of the ray.
    pub fn ray_color(self, world : &HitList) -> Color {
        let mut rec = HitRecord::default();
        if world.hit(&self, 0f32, constants::INFINITY, &mut rec){
            return 0.5f32 * (Color::new(1f32, 1f32, 1f32) + Color::new(rec.normal.x, rec.normal.y, rec.normal.z));
        }

        let unite_direction = self.direction.unit_vector();
        let a = (1f32 + unite_direction.y) * 0.5;
        (1f32 - a) * Color::new(1f32, 1f32, 1f32) + a * Color::new(0.5, 0.7, 1.0)
    }
}