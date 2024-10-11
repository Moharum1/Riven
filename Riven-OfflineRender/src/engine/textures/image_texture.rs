use std::path::Path;
use image::{DynamicImage, ImageReader, RgbImage};
use crate::engine::base::point::Point3;
use crate::engine::textures::{Texture, TextureType};
use crate::util::color::Color;

#[derive(Clone)]
pub struct ImageTexture {
    texture_image: RgbImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> TextureType {
        let path = Path::new(filename);

        // Open the image file and decode
        let binding = ImageReader::open(path)
            .expect("Didn't find the specified file")
            .decode()
            .expect("Failed to decode image");

        // Try to convert to RgbImage, handle other formats gracefully
        let image = match binding {
            DynamicImage::ImageRgb8(img) => img,
            DynamicImage::ImageLuma8(img) => DynamicImage::ImageLuma8(img).to_rgb8(), // Convert Grayscale to RGB
            DynamicImage::ImageLumaA8(img) => DynamicImage::ImageLumaA8(img).to_rgb8(), // Convert Grayscale+Alpha to RGB
            _ => panic!("Unsupported image format"),
        };

        TextureType::Image(Self {
            texture_image: image,
        })
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _: Point3) -> Color {
        // Clamp UV coordinates to [0.0, 1.0] range
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0); // Flip V to match image coordinates


        // Calculate pixel coordinates
        let i = (u * self.texture_image.width() as f32) - 1.0;
        let j = (v * self.texture_image.height() as f32) - 1.0;
        let pixel = self.texture_image.get_pixel(i as u32, j as u32);

        // println!("{:?} {:?}", i, j);

        // Normalize the color values to [0.0, 1.0] range
        Color::new(
            pixel[0] as f32 / 255.0,
            pixel[1] as f32 / 255.0,
            pixel[2] as f32 / 255.0
        )

    }
}
