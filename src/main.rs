mod graphic_controller;
mod images_comparator;
mod stamp_generator;

use std::time::Duration;
use image::{ImageBuffer, Rgba};
use macroquad::prelude::*;
use graphic_controller::GraphicController;
use images_comparator::ImagesComparator;
use stamp_generator::{Stamp, StampGenerator};
use macroquad_canvas::Canvas2D;


#[macroquad::main("BasicShapes")]
async fn main() {
    let image_path: &str = "image.jpeg";
    let loaded_image: ImageBuffer<Rgba<u8>, Vec<u8>> = image::open(image_path)
            .expect("Could not find test-image")
            .into_rgba8();
    let image_size: (f32, f32) = (loaded_image.width() as f32, loaded_image.height() as f32);
    
    let gc = GraphicController::new(
        image_size.0,
        image_size.1
    ).await;
    let ic = ImagesComparator::new(
        loaded_image
    );
    let mut sg: StampGenerator = StampGenerator::new(
        image_size.0,
        image_size.1
    );

    let canvas = Canvas2D::new(image_size.0, image_size.1);

    set_camera(&canvas.camera);
    clear_background(WHITE);
    set_default_camera();
    next_frame().await;

    for i in 1..100 {
        let stamp: Stamp = sg.generate_stamp();
        gc.draw(&stamp, &canvas).await;

        let second_image = gc.extract_image(&canvas);
        let score: f64 = ic.compare_loaded_image_to(second_image);
        println!("Simmilarity score: {}", score);

        let ten_millis = Duration::from_millis(1);
        std::thread::sleep(ten_millis);
    }
}
