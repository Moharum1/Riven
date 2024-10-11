use crate::engine::base::point::Point3;
use crate::engine::textures::{Texture, TextureType};
use crate::engine::textures::TextureType::ChessBoard;
use crate::util::color::Color;

#[derive(Clone)]
pub struct ChessBoardTexture{
    inv_scale : f32,
    even : Box<TextureType>,
    odd : Box<TextureType>
}

impl Texture for ChessBoardTexture{
    fn value(&self, u: f32, v: f32, point: Point3) -> Color {
        let x_int = (self.inv_scale * point.x).floor() as i32;
        let y_int = (self.inv_scale * point.y).floor() as i32;
        let z_int = (self.inv_scale * point.z).floor() as i32;

        let is_even = (x_int + y_int + z_int) % 2 == 0;

        return if is_even {
            self.even.value(u,v, point)
        }else {
            self.odd.value(u,v, point)
        }
    }
}

impl ChessBoardTexture{
    pub fn new(scale : f32, even : TextureType, odd : TextureType) -> TextureType {
        ChessBoard(Self{
            inv_scale : 1.0 / scale,
            even : Box::new(even),
            odd  : Box::new(odd)
        })
    }
}