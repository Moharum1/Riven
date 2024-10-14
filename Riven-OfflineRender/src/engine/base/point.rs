use std::ops::{Add, Div, Index, Mul, Sub};
use rand::distr::uniform::SampleBorrow;
use crate::engine::base::co_ordinate::{CoOrdinateType};
use crate::engine::base::vector::Vector3;

/// A 3D point with x, y, and z components.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point3 {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub z: f32,
    kind: CoOrdinateType,
}

impl Point3{
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
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
            kind: CoOrdinateType::Point,
        }
    }

    /// Computes the length of the vector.
    ///
    /// # Returns
    ///
    /// The length as a `f32`.
   #[inline]
    fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }

    /// Computes the squared length of the vector.
    ///
    /// # Returns
    ///
    /// The squared length as a `f32`.
    #[inline]
    fn len_squared(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    /// Normalizes the vector to a unit vector.
    ///
    /// # Returns
    ///
    /// A new `Vector3` representing the unit vector.
    #[inline]
    pub(crate) fn unit_vector(&self) -> Point3 {
        self / self.len()
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

impl Div<f32> for &Point3 {
    type Output = Point3;

    fn div(self, scale: f32) -> Point3 {
        if scale == 0.0 {
            panic!("Division by zero is not allowed");
        }
        Point3 {
            x: self.x / scale,
            y: self.y / scale,
            z: self.z / scale,
            kind: CoOrdinateType::Point,
        }
    }
}

impl Index<i32> for Point3{
    type Output = f32;

    fn index(&self, index: i32) -> &Self::Output {
        match index {
            1 => self.y.borrow(),
            2 => self.z.borrow(),
            _ => self.x.borrow()
        }
    }
}

impl Mul<Point3> for f32 {
    type Output = Point3;

    fn mul(self, rhs: Point3) -> Self::Output {
        Point3::new(
            rhs.x * self,
            rhs.y * self,
            rhs.z * self
        )
    }
}

impl Add<Point3> for f32 {
    type Output = Point3;

    fn add(self, rhs: Point3) -> Self::Output {
        Point3::new(
            rhs.x + self,
            rhs.y + self,
            rhs.z + self
        )
    }
}

