use std::path::Path;
use image::{Rgba, RgbaImage};
use crate::util::color::Color;

// Some constant Aspect Ratio
const ASPECT_RATIO: f32 = 16.0 / 9.0;


/// Canvas is the Class responsible for saving Images to the Desktop
pub struct Canvas{
    pub width  : u32,
    pub height : u32,
    image : RgbaImage
}


impl Canvas {
    /// Creates a new `Canvas` with the specified width and height.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the canvas.
    /// * `height` - The height of the canvas.
    ///
    /// # Returns
    ///
    /// A new instance of `Canvas`.
    pub fn new(width: u32) -> Self {
        let height = (width as f32 / ASPECT_RATIO) as u32;
        let image = RgbaImage::new(width, height);

        Self {
            width,
            height,
            image,
        }
    }

    /// Saves the current image to the specified file path.
    ///
    /// # Arguments
    ///
    /// * `image_name` - The name of the file to save the image as.
    ///
    /// # Panics
    ///
    /// This function will panic if the image cannot be saved.
    pub fn save_image(&self, image_name: String) {
        let path = Path::new(&image_name);
        self.image.save(path).expect("Image couldn't be saved");
    }

    /// Writes a pixel to the canvas at the specified coordinates with the given color.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the pixel.
    /// * `y` - The y-coordinate of the pixel.
    /// * `r` - The red component of the color.
    /// * `g` - The green component of the color.
    /// * `b` - The blue component of the color.
    pub fn write_pixel(&mut self, x: u32, y: u32, color : Color) {

        if x > self.width || y > self.height {
            panic!("Pixel is out of bounds at location {x},{y}");
        }
        self.image.put_pixel(x, y, Rgba::from(color.get_rgba()));
    }
}


#[cfg(test)]
mod image_test {
    use std::path::Path;
    use image::Rgba;
    use crate::util::image::Canvas;
    use crate::util::color::Color;

    #[test]
    fn create_canvas_with_valid_dimensions() {
        let width = 100;
        let height = 100;
        let canvas = Canvas::new(width);
        assert_eq!(canvas.width, width);
        assert_eq!(canvas.height, height);
    }

    #[test]
    fn save_image_successfully() {
        let canvas = Canvas::new(100);
        canvas.save_image("test_save.png".to_string());
        assert!(Path::new("test_save.png").exists());
        std::fs::remove_file("test_save.png").unwrap();
    }

    #[test]
    fn write_pixel_within_bounds() {
        let mut canvas = Canvas::new(100);
        let color = Color::new(1.0, 0.0, 0.0);
        canvas.write_pixel(50, 50, color);
        let pixel = canvas.image.get_pixel(50, 50);
        assert_eq!(pixel, &Rgba([255, 0, 0, 255]));
    }

    #[test]
    #[should_panic(expected = "Pixel is out of bounds")]
    fn write_pixel_out_of_bounds() {
        let mut canvas = Canvas::new(100);
        let color = Color::new(1.0, 0.0, 0.0);
        canvas.write_pixel(150, 150, color);
    }

    #[test]
    fn check_image_save() {
        let height = 800;

        let mut canvas = Canvas::new(height);
        let width = canvas.width;

        for i in 0..width {
            for j in 0..height {
                let r = i as f32 / width as f32;
                let g = j as f32 / height as f32;
                let b = 0.5;

                let color = Color::new(r, g, b);
                canvas.write_pixel(i, j, color);
            }
        }

        canvas.save_image("test.png".to_string());
    }
}