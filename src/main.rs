use macroquad::prelude::*;
use image::GenericImageView; // For getting the dimensions of the image

// Function that returns the configuration for the window
fn window_conf() -> Conf {
    // Load the image to get its dimensions
    let image = image::open("image.jpeg").expect("Could not find test-image");
    let (image_width, image_height) = image.dimensions();

    // Return the Conf struct with the image dimensions
    Conf {
        window_title: "BasicShapes".to_owned(),
        window_width: image_width as i32,
        window_height: image_height as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(BLACK);

        // Draw some shapes within the image dimensions
        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        // draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);

        // Capture the screen and compare the image as before
        let screen_data = get_screen_data();
        let raw_pixels = screen_data.bytes.chunks(4)
            .flat_map(|pixel| pixel.iter().cloned())
            .collect::<Vec<_>>();

        let image_two = image::RgbaImage::from_raw(screen_data.width as u32, screen_data.height as u32, raw_pixels)
            .expect("Failed to create RgbaImage from screen data");

        let image_one = image::open("image.jpeg").expect("Could not find test-image").into_rgba8();

        let white = image::Rgb([255, 255, 255]);
        let result = image_compare::rgba_blended_hybrid_compare((&image_one).into(), (&image_two).into(), white)
            .expect("Images had different dimensions");
        
        println!("{}", result.score);

        // Continue to the next frame
        next_frame().await;
    }
}
