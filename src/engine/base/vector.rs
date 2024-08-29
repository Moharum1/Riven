use std::ops::{Add, Div, Mul, Sub};
use crate::engine::base::co_ordinate::{CoOrdinate, CoOrdinateType};

/// A 3D vector with x, y, and z components.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Vector3 {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
    pub(crate) kind: CoOrdinateType,
}

impl CoOrdinate for Vector3 {
    /// Creates a new `Vector3` with the specified x, y, and z components.
    ///
    /// # Arguments
    ///
    /// * `x` - The x component.
    /// * `y` - The y component.
    /// * `z` - The z component.
    ///
    /// # Returns
    ///
    /// A new instance of `Vector3`.
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
            kind: CoOrdinateType::Vector,
        }
    }
}

impl Vector3 {
    /// Computes the dot product of this vector and another vector.
    ///
    /// # Arguments
    ///
    /// * `other` - The other vector.
    ///
    /// # Returns
    ///
    /// The dot product as a `f32`.
    fn dot(&self, other: &Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Computes the cross product of this vector and another vector.
    ///
    /// # Arguments
    ///
    /// * `other` - The other vector.
    ///
    /// # Returns
    ///
    /// A new `Vector3` representing the cross product.
    fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            kind: CoOrdinateType::Vector,
        }
    }

    /// Computes the length of the vector.
    ///
    /// # Returns
    ///
    /// The length as a `f32`.
    fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }

    /// Computes the squared length of the vector.
    ///
    /// # Returns
    ///
    /// The squared length as a `f32`.
    fn len_squared(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    /// Normalizes the vector to a unit vector.
    ///
    /// # Returns
    ///
    /// A new `Vector3` representing the unit vector.
    fn unit_vector(&self) -> Vector3 {
        self / self.len()
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            kind: CoOrdinateType::Vector,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            kind: CoOrdinateType::Vector,
        }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            kind: CoOrdinateType::Vector,
        }
    }
}

impl Div<f32> for Vector3 {
    type Output = Self;

    fn div(self, scale: f32) -> Self {
        if scale == 0.0 {
            panic!("Division by zero is not allowed");
        }
        Self {
            x: self.x / scale,
            y: self.y / scale,
            z: self.z / scale,
            kind: CoOrdinateType::Vector,
        }
    }
}

impl Div<f32> for &Vector3 {
    type Output = Vector3;

    fn div(self, scale: f32) -> Vector3 {
        if scale == 0.0 {
            panic!("Division by zero is not allowed");
        }
        Vector3 {
            x: self.x / scale,
            y: self.y / scale,
            z: self.z / scale,
            kind: CoOrdinateType::Vector,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_vector() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn vector_addition() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        let result = v1 + v2;
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 7.0);
        assert_eq!(result.z, 9.0);
    }

    #[test]
    fn vector_subtraction() {
        let v1 = Vector3::new(4.0, 5.0, 6.0);
        let v2 = Vector3::new(1.0, 2.0, 3.0);
        let result = v1 - v2;
        assert_eq!(result.x, 3.0);
        assert_eq!(result.y, 3.0);
        assert_eq!(result.z, 3.0);
    }

    #[test]
    fn vector_multiplication() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        let result = v * 2.0;
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 4.0);
        assert_eq!(result.z, 6.0);
    }

    #[test]
    #[should_panic(expected = "Division by zero is not allowed")]
    fn vector_division_by_zero() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        let _ = v / 0.0;
    }

    #[test]
    fn vector_division() {
        let v = Vector3::new(2.0, 4.0, 6.0);
        let result = v / 2.0;
        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, 2.0);
        assert_eq!(result.z, 3.0);
    }

    #[test]
    fn vector_dot_product() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        let result = v1.dot(&v2);
        assert_eq!(result, 32.0);
    }

    #[test]
    fn vector_cross_product() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        let result = v1.cross(&v2);
        assert_eq!(result.x, -3.0);
        assert_eq!(result.y, 6.0);
        assert_eq!(result.z, -3.0);
    }

    #[test]
    fn vector_length() {
        let v = Vector3::new(1.0, 2.0, 2.0);
        let result = v.len();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn vector_unit_vector() {
        let v = Vector3::new(1.0, 2.0, 2.0);
        let result = v.unit_vector();
        assert_eq!(result.x, 1.0 / 3.0);
        assert_eq!(result.y, 2.0 / 3.0);
        assert_eq!(result.z, 2.0 / 3.0);
    }
}