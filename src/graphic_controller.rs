use macroquad::prelude::*;


pub struct GraphicController { }

impl GraphicController {
    pub async fn new(init_width: f32, init_height: f32) -> Self {
        Self::set_screen_size(init_width, init_height).await;
        Self { }
    }

    async fn set_screen_size(width: f32, height: f32) -> () {
        request_new_screen_size(width, height);
        next_frame().await;
    }

    pub async fn draw(&self) {
        clear_background(BLACK);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);

        next_frame().await;
    }

    pub fn extract_image(&self) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
        let screen_data: Image = get_screen_data();
        let raw_pixels = screen_data.bytes.chunks(4)
            .flat_map(|pixel| pixel.iter().cloned())
            .collect::<Vec<_>>();
        let extracted_image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = image::RgbaImage::from_raw(screen_data.width as u32, screen_data.height as u32, raw_pixels)
            .expect("Failed to create RgbaImage from screen data");
        extracted_image
    }

}