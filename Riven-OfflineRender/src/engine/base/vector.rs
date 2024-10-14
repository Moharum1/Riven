use std::ops::{Add, Div, Index, Mul, Neg, Sub};
use rand::distr::uniform::SampleBorrow;
use crate::engine::base::co_ordinate::{CoOrdinateType};
use crate::engine::base::constants::constants;
use crate::engine::base::constants::constants::random_float;

/// A 3D vector with x, y, and z components.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vector3 {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
    pub(crate) kind: CoOrdinateType,
}
impl Vector3 {

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
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
            kind: CoOrdinateType::Vector,
        }
    }



    /// Computes the dot product of this vector and another vector.
    ///
    /// # Arguments
    ///
    /// * `other` - The other vector.
    ///
    /// # Returns
    ///
    /// The dot product as a `f32`.
    #[inline]
    pub(crate) fn dot(&self, other: &Vector3) -> f32 {
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
    #[inline]
    pub(crate) fn cross(&self, other: &Vector3) -> Vector3 {
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
    #[inline]
    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }

    /// Computes the squared length of the vector.
    ///
    /// # Returns
    ///
    /// The squared length as a `f32`.
    #[inline]
    pub fn len_squared(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    /// Normalizes the vector to a unit vector.
    ///
    /// # Returns
    ///
    /// A new `Vector3` representing the unit vector.
    #[inline]
    pub(crate) fn unit_vector(&self) -> Vector3 {
        self / self.len()
    }

    pub fn create_random_unit_vector() -> Vector3 {
        Vector3::new(
            random_float(),
            random_float(),
            random_float()
        )
    }

    pub fn create_random_vector_with_bound(min : f32, max : f32) -> Vector3 {
        Vector3::new(
            constants::ranged_random_float(min, max),
            constants::ranged_random_float(min, max),
            constants::ranged_random_float(min, max)
        )
    }

    #[inline]
    pub fn random_in_unit_sphere() -> Vector3 {
        loop {
            let p = Vector3::create_random_vector_with_bound(-1.0, 1.0);
            if p.len_squared() < 1.0 {
                return p;
            }
        }
    }

    #[inline]
    pub fn random_unit_vector() -> Vector3 {
        Self::random_in_unit_sphere().unit_vector()
    }

    #[inline]
    pub fn random_in_hemisphere(normal: &Vector3) -> Vector3 {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub(crate) fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    #[inline]
    pub fn reflect(&self , normal: &Vector3) -> Vector3 {
        *self - normal * 2.0 * self.dot(normal)
    }

    #[inline]
    pub fn refract(&self, normal: &Vector3, etai_over_etat: f32) -> Vector3 {
        let cos_theta = (-*self).dot(normal).min(1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * *normal);
        let r_out_parallel = -(1.0 - r_out_perp.len_squared()).abs().sqrt() * *normal;
        r_out_perp + r_out_parallel
    }

    #[inline]
    pub fn random_in_unit_disk() -> Vector3 {
        loop {
            let p = Vector3::new(random_float(), random_float(), 0f32);
            if p.len_squared() < 1f32{
                return p;
            }
        }
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

impl Mul<Vector3> for f32 {
    type Output = Vector3;

    /// Multiplies a scalar by a vector.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The vector to be multiplied.
    ///
    /// # Returns
    ///
    /// A new `Vector3` resulting from the multiplication.
    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3::new(self * rhs.x, self * rhs.y, self * rhs.z)
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

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            kind: CoOrdinateType::Vector,
        }
    }
}

impl Mul<f64> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3 {
            x: self.x * rhs as f32,
            y: self.y * rhs as f32,
            z: self.z * rhs as f32,
            kind: CoOrdinateType::Vector,
        }
    }
}

impl Index<i32> for Vector3{
    type Output = f32;

    fn index(&self, index: i32) -> &f32 {
        match index {
            1 => self.y.borrow(),
            2 => self.z.borrow(),
            _ => self.x.borrow()
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