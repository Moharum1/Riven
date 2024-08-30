use std::ops::Mul;
use crate::engine::base::constants::constants;
use crate::engine::base::interval::Interval;
use crate::engine::base::point::Point3;
use crate::engine::base::ray::Ray;
use crate::engine::base::vector::Vector3;
use crate::engine::objects::object::{HitList, HitRecord, Object};
use crate::util::color::Color;
use crate::util::image::Canvas;

/// A struct representing an RGB camera.
pub struct RGBCamera {
    /// The aspect ratio of the camera.
    pub aspect_ratio: f32,
    /// The width of the image.
    pub image_width: u32,
    /// The number of samples per pixel.
    pub samples_per_pixel: i32,

    /// The height of the image.
    image_height: u32,
    /// The center point of the camera.
    center: Point3,
    /// The location of the upper-left pixel.
    pixel00_location: Point3,
    /// The offset to the pixel to the right.
    pixel_delta_u: Vector3,
    /// The offset to the pixel below.
    pixel_delta_v: Vector3,
    /// The scale factor for the sum of pixel samples.
    pixel_sample_scale: f32,
}

impl Mul<Vector3> for f32 {
    type Output = Vector3;

    /// Multiplies a scalar by a vector.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The vector to be multiplied.
    ///
    /// # Returns
    ///
    /// A new `Vector3` resulting from the multiplication.
    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl RGBCamera {
    /// Initializes the camera parameters.
    fn initialize(&mut self) {
        self.image_height = (self.image_width as f32 / self.aspect_ratio) as u32;

        // Pixel Sample Scale
        self.pixel_sample_scale = 1.0 / self.samples_per_pixel as f32;

        // ViewPort
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f32 / self.image_height as f32);
        self.center = Point3::new(0f32, 0f32, 0f32);

        let viewport_u = Vector3::new(viewport_width, 0f32, 0f32);
        let viewport_v = Vector3::new(0f32, -viewport_height, 0f32);

        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        // Calculate the location of the upper left pixel
        let upper_left_pixel = self.center - Vector3::new(0f32, 0f32, focal_length) - viewport_u / 2f32 - viewport_v / 2f32;
        self.pixel00_location = upper_left_pixel + self.pixel_delta_u / 2f32 + self.pixel_delta_v / 2f32;
    }

    /// Computes the color of a ray based on its interaction with the world.
    ///
    /// # Arguments
    ///
    /// * `ray` - The ray to be traced.
    /// * `world` - The world containing objects to be hit by the ray.
    ///
    /// # Returns
    ///
    /// A `Color` representing the color of the ray.
    fn ray_color(ray: &Ray, world: &HitList) -> Color {
        let mut rec = HitRecord::default();
        if world.hit(ray, Interval::new(0f32, constants::INFINITY), &mut rec) {
            return 0.5f32 * (Color::new(1f32, 1f32, 1f32) + Color::new(rec.normal.x, rec.normal.y, rec.normal.z));
        }

        let unite_direction = ray.direction.unit_vector();
        let a = (1f32 + unite_direction.y) * 0.5;
        (1f32 - a) * Color::new(1f32, 1f32, 1f32) + a * Color::new(0.5, 0.7, 1.0)
    }

    /// Samples a random point within a unit square.
    ///
    /// # Returns
    ///
    /// A `Vector3` representing the sampled point.
    fn sample_square(&self) -> Vector3 {
        Vector3::new(constants::random_float() - 0.5, constants::random_float() - 0.5, 0f32)
    }

    /// Constructs a camera ray originating from the camera center and directed at a randomly sampled point around the pixel location (i, j).
    ///
    /// # Arguments
    ///
    /// * `i` - The x-coordinate of the pixel.
    /// * `j` - The y-coordinate of the pixel.
    ///
    /// # Returns
    ///
    /// A `Ray` representing the camera ray.
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_location + ((i as f32 + offset.x) * self.pixel_delta_u) + ((j as f32 + offset.y) * self.pixel_delta_v);

        // The camera ray is the ray from the camera center to the pixel location
        let ray_direction = pixel_sample - self.center;
        Ray::new(self.center, ray_direction)
    }

    /// Renders the scene by tracing rays through each pixel and computing the color.
    ///
    /// # Arguments
    ///
    /// * `world` - The world containing objects to be hit by the rays.
    /// * `canvas` - The canvas to write the pixel colors to.
    pub fn render(&mut self, world: &HitList, mut canvas: Canvas) {
        self.initialize();

        for i in 0..self.image_width {
            for j in 0..self.image_height {
                let mut pixel_color = Color::new(0f32, 0f32, 0f32);

                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color = pixel_color + Self::ray_color(&ray, &world);
                }

                canvas.write_pixel(i, j, self.pixel_sample_scale * pixel_color);
            }
        }

        canvas.save_image("test.png".to_string());
    }
}

impl Default for RGBCamera {
    /// Provides default values for the `RGBCamera` struct.
    ///
    /// # Returns
    ///
    /// A new instance of `RGBCamera` with default values.
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 400,
            samples_per_pixel: 10,
            image_height: 400,
            center: Point3::default(),
            pixel00_location: Point3::default(),
            pixel_delta_u: Vector3::default(),
            pixel_delta_v: Vector3::default(),
            pixel_sample_scale: 1.0,
        }
    }
}