use crate::engine::base::point::Point3;
use crate::engine::base::vector::Vector3;


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
}