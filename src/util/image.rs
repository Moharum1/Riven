use std::path::Path;
use image::{Rgba, RgbaImage};

/// Canvas is the Class responsible for saving Images to the Desktop
struct Canvas{
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
    fn new(width: u32, height: u32) -> Self {
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
    fn save_image(&self, image_name: String) {
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
    fn write_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8) {

        if x > self.width || y > self.height {
            panic!("Pixel is out of bounds at location {x},{y}");
        }

        self.image.put_pixel(x, y, Rgba::from([r, g, b, 255u8]));
    }
}


#[cfg(test)]
mod image_test {
    use std::path::Path;
    use image::Rgba;
    use crate::util::image::Canvas;

    #[test]
    fn create_canvas_with_valid_dimensions() {
        let width = 100;
        let height = 100;
        let canvas = Canvas::new(width, height);
        assert_eq!(canvas.width, width);
        assert_eq!(canvas.height, height);
    }

    #[test]
    fn save_image_successfully() {
        let canvas = Canvas::new(100, 100);
        canvas.save_image("test_save.png".to_string());
        assert!(Path::new("test_save.png").exists());
        std::fs::remove_file("test_save.png").unwrap();
    }

    #[test]
    fn write_pixel_within_bounds() {
        let mut canvas = Canvas::new(100, 100);
        canvas.write_pixel(50, 50, 255, 0, 0);
        let pixel = canvas.image.get_pixel(50, 50);
        assert_eq!(pixel, &Rgba([255, 0, 0, 0]));
    }


    //
    // #[test]
    // fn save_image_with_invalid_path() {
    //     let canvas = Canvas::new(100, 100);
    //     canvas.save_image("invalid_path/test_save.png".to_string());
    // }

    #[test]
    #[should_panic(expected = "Pixel is out of bounds")]
    fn write_pixel_out_of_bounds() {
        let mut canvas = Canvas::new(100, 100);
        canvas.write_pixel(150, 150, 255, 0, 0);
    }

    #[test]
    fn check_image_save(){
        let width = 256;
        let height = 256;

        let mut canvas = Canvas::new(width, height);

        for i  in 0..width {
            for j in 0..height {
                let r = i * 256/ width;
                let g = j * 256/ height;
                let b = r / (g + 1);

                canvas.write_pixel(i, j, r as u8, g as u8, b as u8);
            }
        }

        canvas.save_image("test.png".to_string());
    }
}
