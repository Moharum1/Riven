use minifb::{Key, KeyRepeat, Window, WindowOptions};
use rayon::prelude::*;
use Riven_OfflineRender::engine::base::point::Point3;
use Riven_OfflineRender::engine::base::vector::Vector3;
use Riven_OfflineRender::engine::camera::rgb_camera::RGBCamera;
use Riven_OfflineRender::engine::objects::object::HitList;
use Riven_OfflineRender::util::color::Color;


pub struct RealTimeWindow{
    pub(crate) buffer : Vec<u32>,
    pub(crate) width : usize,
    pub(crate) height : usize,
    pub(crate) window: Window
}

impl RealTimeWindow{

    pub fn default_window() -> Self {
        let width = 800;
        let height = (width as f32 / (16.0 / 9.0)) as usize;

        let buffer: Vec<u32> = vec![0; width * height];
        let mut window = Window::new(
            "RealTimeCam",
            width,
            height,
            WindowOptions::default(),
        ).unwrap_or_else(|e| {
            panic!("{}", e);
        });

        window.set_target_fps(30);


        Self{
            buffer,
            width,
            height,
            window,
        }
    }

    pub fn update(&mut self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    pub fn handle_input(&self,cam: &mut RGBCamera){
        self.window.get_keys_pressed(KeyRepeat::No).iter().for_each(|key|{
            match key {
                Key::W => {
                    cam.look_from = cam.look_from - Vector3::new(0.0, 0.0, 5.0) ;
                    cam.update_orientation()
                }
                Key::S => {
                    cam.look_from = cam.look_from + Vector3::new(0.0, 0.0, 5.0) ;
                    cam.update_orientation()
                }

                Key::A => {
                   cam.look_from = cam.look_from - Vector3::new(5.0, 0.0, 0.0) ;
                    cam.update_orientation()
                }

                Key::D => {
                    cam.look_from = cam.look_from + Vector3::new(5.0, 0.0, 0.0) ;
                    cam.update_orientation()
                }
                _ => {
                    println!("Other key pressed");
                }
            }
        })
    }

    pub fn render(&mut self, world: &HitList, cam: &mut RGBCamera) {
        cam.initialize();


        while self.update() {
            self.handle_input(cam);
            // Split the buffer into chunks to process more pixels at once
            self.buffer.par_chunks_mut(self.width * 10) // Each chunk is 10 rows
                .enumerate()
                .for_each(|(chunk_idx, chunk)| {
                    let start_y = chunk_idx * 10;

                    for (y, row) in chunk.chunks_mut(self.width).enumerate() {
                        let actual_y = start_y + y;

                        for (x, pixel) in row.iter_mut().enumerate() {
                            let mut pixel_color = Color::default();

                            for _ in 0..cam.samples_per_pixel {
                                let ray = cam.get_ray(x as u32, actual_y as u32);
                                pixel_color = pixel_color + RGBCamera::ray_color(&ray, world, cam.max_depth);
                            }

                            pixel_color = cam.pixel_sample_scale * pixel_color;
                            *pixel = pixel_color.to_hexa_value();
                        }
                    }
                });

            // Update the frame
            self.window.update_with_buffer(&self.buffer, self.width, self.height).unwrap();
        }
    }

}


