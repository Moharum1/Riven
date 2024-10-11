use crate::engine::base::point::Point3;
use crate::engine::textures::{Texture, TextureType};
use crate::engine::textures::TextureType::NormalColor;
use crate::util::color::Color;

#[derive(Clone, Default)]
pub struct SolidColor{
    albedo : Color
}

impl Texture for SolidColor{
    fn value(&self, _: f32, _: f32, _: Point3) -> Color {
        self.albedo
    }
}

impl<'a> SolidColor{
    pub const fn new(color: Color) -> TextureType{
        NormalColor(Self{
            albedo: color
        })
    }

    pub fn from_rgb(r : f32, g : f32, b : f32) -> TextureType {
        NormalColor(
            Self{
                albedo : Color::new(r, g, b)
            }
        )
    }
}
