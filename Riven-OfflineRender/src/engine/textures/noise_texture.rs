use std::ops::{Add, Mul};
use crate::engine::base::point::Point3;
use crate::engine::textures::noise::perlin_noise::PerlinNoise;
use crate::engine::textures::{Texture, TextureType};
use crate::util::color::Color;

#[derive(Clone)]
pub struct NoiseTexture{
    noise : PerlinNoise,
    scale : f32
}

impl NoiseTexture{
    pub fn new(scale : f32) -> TextureType {
        TextureType::Noise(
            Self {
                noise: PerlinNoise::new(),
                scale
            }
        )
    }
}


impl Texture for NoiseTexture{
    fn value(&self, _: f32, _: f32, point: Point3) -> Color {
        let noise_value = 1.0 + (self.scale * point.z + 10.0 * self.noise.turb(point, 7)).sin();
        noise_value * Color::new(0.5, 0.5, 0.5)
    }
}