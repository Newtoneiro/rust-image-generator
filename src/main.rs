mod graphic_controller;
mod images_comparator;

use std::time::Duration;
use image::{ImageBuffer, Rgba};
use macroquad::prelude::*;
use graphic_controller::GraphicController;
use images_comparator::ImagesComparator;


#[macroquad::main("BasicShapes")]
async fn main() {
    let image_path: &str = "image.jpeg";
    let loaded_image: ImageBuffer<Rgba<u8>, Vec<u8>> = image::open(image_path)
            .expect("Could not find test-image")
            .into_rgba8();
    
    let gc = GraphicController::new(
        loaded_image.width() as f32, loaded_image.height() as f32
    ).await;
    let ic = ImagesComparator::new(
        loaded_image
    );

    // Print the screen size
    
    gc.draw().await;

    let second_image = gc.extract_image();
    let score: f64 = ic.compare_loaded_image_to(second_image);
    println!("Simmilarity score: {}", score);

    let ten_millis = Duration::from_millis(1000);
    std::thread::sleep(ten_millis);
}
