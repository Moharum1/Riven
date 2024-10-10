use crate::engine::base::constants::constants;

/// Represents an interval with a minimum and maximum value.
#[derive(Clone)]
pub struct Interval {
    /// The minimum value of the interval.
    pub(crate) min: f32,
    /// The maximum value of the interval.
    pub(crate) max: f32,
}

impl Interval {
    /// Creates a new `Interval` with the specified minimum and maximum values.
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum value of the interval.
    /// * `max` - The maximum value of the interval.
    ///
    /// # Returns
    ///
    /// A new `Interval` instance.
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn from_interval(interval1 : Interval, interval2 : Interval) -> Interval {
        Self{
            min: if interval1.min <= interval2.min {interval1.min}else {interval2.min},
            max: if interval1.max <= interval2.max {interval1.max}else {interval2.max},
        }
    }

    /// Returns the size of the interval.
    ///
    /// # Returns
    ///
    /// The size of the interval as a `f32`.
    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    /// Checks if a value is contained within the interval.
    ///
    /// # Arguments
    ///
    /// * `x` - The value to check.
    ///
    /// # Returns
    ///
    /// `true` if the value is within the interval, `false` otherwise.
    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    /// Checks if a value is strictly within the interval (not on the boundaries).
    ///
    /// # Arguments
    ///
    /// * `x` - The value to check.
    ///
    /// # Returns
    ///
    /// `true` if the value is strictly within the interval, `false` otherwise.
    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub fn expand(&self, delta : f32) -> Interval {
        Interval::new(self.min - delta, self.max + delta)
    }
}

impl Default for Interval {
    /// Provides a default `Interval` with `min` set to positive infinity and `max` set to negative infinity.
    ///
    /// # Returns
    ///
    /// A default `Interval` instance.
    fn default() -> Self {
        Self {
            min: constants::INFINITY,
            max: -constants::INFINITY,
        }
    }
}

/// An empty interval with `min` set to positive infinity and `max` set to negative infinity.
const EMPTY: Interval = Interval {
    min: constants::INFINITY,
    max: -constants::INFINITY,
};

/// A universal interval with `min` set to negative infinity and `max` set to positive infinity.
const UNIVERSE: Interval = Interval {
    min: -constants::INFINITY,
    max: constants::INFINITY,
};