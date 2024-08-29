use std::ops::{Add, Sub};
use crate::engine::base::co_ordinate::{CoOrdinate, CoOrdinateType};
use crate::engine::base::vector::Vector3;

/// A 3D point with x, y, and z components.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point3 {
    x: f32,
    y: f32,
    z: f32,
    kind: CoOrdinateType,
}

impl CoOrdinate for Point3 {
    /// Creates a new `Point3` with the specified x, y, and z components.
    ///
    /// # Arguments
    ///
    /// * `x` - The x component.
    /// * `y` - The y component.
    /// * `z` - The z component.
    ///
    /// # Returns
    ///
    /// A new instance of `Point3`.
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
            kind: CoOrdinateType::Point,
        }
    }
}

impl Add<Vector3> for Point3 {
    type Output = Self;

    fn add(self, vector: Vector3) -> Self {
        Self {
            x: self.x + vector.x,
            y: self.y + vector.y,
            z: self.z + vector.z,
            kind: CoOrdinateType::Point,
        }
    }
}

impl Sub<Vector3> for Point3 {
    type Output = Self;

    fn sub(self, vector: Vector3) -> Self {
        Self {
            x: self.x - vector.x,
            y: self.y - vector.y,
            z: self.z - vector.z,
            kind: CoOrdinateType::Point,
        }
    }
}

impl Sub<Point3> for Point3 {
    type Output = Vector3;

    fn sub(self, point: Point3) -> Vector3 {
        Vector3 {
            x: self.x - point.x,
            y: self.y - point.y,
            z: self.z - point.z,
            kind: CoOrdinateType::Vector,
        }
    }
}