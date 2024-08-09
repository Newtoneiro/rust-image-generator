use macroquad::prelude::*;
use image::GenericImageView;
use graphic_controller::GraphicController; // For getting the dimensions of the image

mod graphic_controller;


#[macroquad::main("BasicShapes")]
async fn main() {
    let mut gc: GraphicController = GraphicController::new("image.jpeg");
    
    loop {
        gc.draw();
       
        let screen_data: Image = gc.get_screen_data();
        let raw_pixels = screen_data.bytes.chunks(4)
            .flat_map(|pixel| pixel.iter().cloned())
            .collect::<Vec<_>>();

        let image_two: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = image::RgbaImage::from_raw(screen_data.width as u32, screen_data.height as u32, raw_pixels)
            .expect("Failed to create RgbaImage from screen data");

        let image_one: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = image::open("image.jpeg").expect("Could not find test-image").into_rgba8();

        let white = image::Rgb([255, 255, 255]);
        let result = image_compare::rgba_blended_hybrid_compare((&image_one).into(), (&image_two).into(), white)
            .expect("Images had different dimensions");
        
        println!("{}", result.score);

    }
}
