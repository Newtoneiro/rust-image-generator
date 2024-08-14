mod graphic_controller;
mod images_comparator;
mod stamp_generator;
mod evolution_algorithm;

use image::{ImageBuffer, Rgba};
use macroquad::prelude::*;
use macroquad_canvas::Canvas2D;
use graphic_controller::GraphicController;
use images_comparator::ImagesComparator;
use stamp_generator::{Stamp, StampGenerator};
use evolution_algorithm::EvolutionAlgorithm;


const NUMBER_OF_ITERATIONS: u16 = 1000;
const IMAGE_PATH: &str = "image.jpg";

#[macroquad::main("ImageGenerator")]
// #[tokio::main]
async fn main() {
    let loaded_image: ImageBuffer<Rgba<u8>, Vec<u8>> = image::open(IMAGE_PATH)
    .expect("Could not find test-image")
    .into_rgba8();
    let image_size: (f32, f32) = (
        loaded_image.width() as f32,
        loaded_image.height() as f32
    );
    let mut sg: StampGenerator = StampGenerator::new(image_size.0, image_size.1);
    let ic: ImagesComparator = ImagesComparator::new(loaded_image);
    let gc: GraphicController = GraphicController::new(
        image_size.0,
        image_size.1
    ).await;
    
    let canvas = Canvas2D::new(image_size.0, image_size.1);
    set_camera(&canvas.camera);
    clear_background(WHITE);
    set_default_camera();
    next_frame().await;
    
    for iteration in 0..NUMBER_OF_ITERATIONS {
        println!("Iteration: {:?}/{:?}", iteration, NUMBER_OF_ITERATIONS);
        let mut ea: EvolutionAlgorithm = EvolutionAlgorithm::new();
        
        let next_stamp: &Stamp = ea.run(
            &canvas.get_texture().clone(),
            &gc,
            &ic,
            &mut sg,
        ).await;
        
        gc.draw(next_stamp, &canvas).await;
        gc.refresh_canvas(&canvas).await;
    }

    println!("Done!");
}
