use crate::engine::base::point::Point3;
use crate::engine::textures::chess_board_texture::ChessBoardTexture;
use crate::engine::textures::image_texture::ImageTexture;
use crate::engine::textures::noise_texture::NoiseTexture;
use crate::engine::textures::solid_color::SolidColor;
use crate::engine::textures::TextureType::{ChessBoard, Image, Noise, NormalColor};
use crate::util::color::Color;

pub mod chess_board_texture;
pub mod solid_color;
pub mod image_texture;
pub mod noise;
pub mod noise_texture;

pub(crate) trait Texture{
    fn value(&self, u : f32, v : f32, point : Point3) -> Color;
}

#[derive(Clone)]
pub enum TextureType{
    NormalColor(SolidColor),
    ChessBoard(ChessBoardTexture),
    Image(ImageTexture),
    Noise(NoiseTexture)
}

impl Texture for TextureType{
    fn value(&self, u: f32, v: f32, point: Point3) -> Color {
        match self {
           NormalColor(sc) => sc.value(u, v, point),
           ChessBoard(CBT) => CBT.value(u, v, point),
           Image(ImageT) => ImageT.value(u, v, point),
           Noise(noise) => noise.value(u, v, point)
        }
    }
}

impl Default for TextureType{
    fn default() -> Self {
        SolidColor::new(Color::default())
    }
}