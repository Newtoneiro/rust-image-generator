use macroquad::prelude::*;
use image::{ImageBuffer, Rgba};

use crate::stamp_generator::Stamp;

pub struct GraphicController {
}

impl GraphicController {
    pub async fn new(init_width: f32, init_height: f32) -> Self {
        let gc = Self { };
        gc.set_screen_size(init_width, init_height).await;
        gc.init_window().await;
        gc
    }

    async fn set_screen_size(&self, width: f32, height: f32) {
        request_new_screen_size(width, height);
        next_frame().await;
    }
    
    async fn init_window(&self) {
        clear_background(WHITE);
        next_frame().await;
    }

    pub async fn draw(&self, stamp: Stamp) {
        let Stamp { char, size, color, pos_x, pos_y } = stamp;
        
        draw_text(
            &char.to_string(),
            pos_x,
            pos_y,
            size,
            color,
        );

        next_frame().await;
    }

    pub fn extract_image(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let screen_data: Image = get_screen_data();
        let raw_pixels = screen_data
            .bytes
            .chunks(4)
            .flat_map(|pixel| pixel.iter().cloned())
            .collect::<Vec<_>>();

        ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(screen_data.width as u32, screen_data.height as u32, raw_pixels)
            .expect("Failed to create RgbaImage from screen data")
    }
}
