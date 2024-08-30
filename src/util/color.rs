use std::ops::{Add, Mul};

pub struct Color{
    pub r : f32,
    pub g : f32,
    pub b : f32,
    pub a : f32
}

impl Color{
    pub fn new(r: f32, g: f32, b: f32) -> Self{
        Self{
            r,
            g,
            b,
            a: 1f32,
        }
    }

    pub fn get_rgba(&self) -> [u8; 4]{
        let r = (self.r * 255f32).clamp(0f32, 255f32) as u8;
        let g = (self.g * 255f32).clamp(0f32, 255f32) as u8;
        let b = (self.b * 255f32).clamp(0f32, 255f32) as u8;

        [r, g, b, 255]
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(self * rhs.r, self * rhs.g, self * rhs.b)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Self{
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: 1f32,
        }
    }
}
