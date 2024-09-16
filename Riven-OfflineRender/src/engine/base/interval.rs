use crate::engine::base::constants::constants;

pub struct Interval{
    pub(crate) min : f32,
    pub(crate) max : f32
}

impl Interval{

    pub fn new(min : f32, max : f32) -> Self {
        Self{
            min,
            max
        }
    }

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn contains(&self, x : f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x : f32) -> bool {
        self.min < x && x < self.max
    }
}

impl Default for Interval{
    fn default() -> Self {
        Self{
            min : constants::INFINITY,
            max : -constants::INFINITY
        }
    }
}

const EMPTY : Interval = Interval{min : constants::INFINITY, max : -constants::INFINITY};
const UNIVERSE : Interval = Interval{min : -constants::INFINITY, max : constants::INFINITY};