use macroquad::prelude::*;


pub struct GraphicController {
    loaded_image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
}


impl GraphicController {
    pub fn new(image_path: &str) -> Self {
        let loaded_image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = image::open(image_path).expect("Could not find test-image").into_rgba8();
        request_new_screen_size(loaded_image.width() as f32, loaded_image.height() as f32);
        Self { loaded_image }
    }

    pub async fn draw(self) -> () {
        clear_background(BLACK);

        // Draw some shapes within the image dimensions
        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);

        next_frame().await;
    }

    pub fn get_screen_data(self) -> Image {
        get_screen_data()
    }
}