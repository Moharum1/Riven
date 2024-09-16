use crate::engine::base::point::Point3;
use crate::util::color::Color;

struct PointLight{
    intensity : Color,
    position  : Point3
}

impl PointLight{
    pub fn new(intensity : Color, position  : Point3) -> PointLight {
        PointLight{
            intensity,
            position
        }
    }
}