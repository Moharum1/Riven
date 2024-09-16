use std::ops::{Add, Mul};
use crate::engine::base::constants::constants::random_float;

#[derive(Debug, Copy, Clone)]
pub struct Color{
    pub r : f32,
    pub g : f32,
    pub b : f32,
    pub a : f32
}


impl Color{
    pub const fn new(r: f32, g: f32, b: f32) -> Self{
        Self{
            r,
            g,
            b,
            a: 1f32,
        }
    }

    pub fn default() -> Color {
        Self::new(0f32, 0f32, 0f32)
    }

    #[inline]
    fn linear_to_gamma(&self, linear_comp : f32) -> f32 {
        if linear_comp > 0f32 {
            linear_comp.sqrt()
        }else {
            0f32
        }
    }

    pub fn get_rgba(&self) -> [u8; 4]{

        let mut r = self.linear_to_gamma(self.r);
        let mut g = self.linear_to_gamma(self.g);
        let mut b = self.linear_to_gamma(self.b);

        r = (r * 255f32).clamp(0f32, 255f32);
        g = (g * 255f32).clamp(0f32, 255f32);
        b = (b * 255f32).clamp(0f32, 255f32);

        [r as u8, g as u8, b as u8, 255]
    }

    #[inline]
    pub fn random() -> Self {
        Self::new(random_float(), random_float(), random_float())
    }

    pub fn to_hexa_value(&self) -> u32 {
        // Clamping values between 0.0 and 1.0
        let r = (self.r.clamp(0.0, 1.0) * 255.0) as u32;
        let g = (self.g.clamp(0.0, 1.0) * 255.0) as u32;
        let b = (self.b.clamp(0.0, 1.0) * 255.0) as u32;

        // Combining the r, g, b values into a single u32 value in hex
        (r << 16) | (g << 8) | b
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

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color{
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
            a: 1f32,
        }
    }
}

