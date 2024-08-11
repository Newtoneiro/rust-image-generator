use macroquad::prelude::*;
use image::{ImageBuffer, Rgba};
use macroquad_canvas::Canvas2D;
use crate::stamp_generator::Stamp;
use macroquad::text::TextParams;


const FONT_SCALE: f32 = 1.0;
const FONT_SCALE_ASPECT: f32 = 1.0;

pub struct GraphicController {
}

impl GraphicController {
    pub async fn new(init_width: f32, init_height: f32) -> Self {
        let gc: GraphicController = Self { };
        
        gc.set_screen_size(init_width, init_height).await;
        
        gc
    }

    async fn set_screen_size(&self, width: f32, height: f32) {
        request_new_screen_size(width, height);
        next_frame().await;
    }

    pub async fn draw(&self, stamp: &Stamp, canvas: &Canvas2D) {
        set_camera(&canvas.camera);

        draw_text_ex(
            &stamp.char.to_string(),
            stamp.pos_x,
            stamp.pos_y,
            self.get_text_params_from_stamp(&stamp),
        );

        self.refresh_canvas(&canvas).await;
    }

    async fn refresh_canvas(&self, canvas: &Canvas2D) -> () {
        set_default_camera();
        canvas.draw();
        next_frame().await;
    }

    fn get_text_params_from_stamp(&self, stamp: &Stamp) -> TextParams {
        TextParams {
            font: None,
            font_size: stamp.size as u16,
            font_scale: FONT_SCALE,
            font_scale_aspect: FONT_SCALE_ASPECT,
            rotation: stamp.rotation,
            color: stamp.color,
        }
    }

    pub fn extract_image(&self, canvas: &Canvas2D) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let texture = canvas.get_texture();
        let texture_data = texture.get_texture_data();
        let raw_pixels = texture_data
            .bytes
            .chunks(4)
            .flat_map(|pixel| pixel.iter().cloned())
            .collect::<Vec<_>>();


        ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(
            texture.width() as u32,
            texture.height()as u32,
            raw_pixels,
        )
            .expect("Failed to create RgbaImage from canvas data")
    }
}
