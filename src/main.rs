mod graphic_controller;
mod images_comparator;
mod stamp_generator;
mod evolution_algorithm;

use image::{ImageBuffer, Rgba};
use macroquad::prelude::*;
use graphic_controller::GraphicController;
use images_comparator::ImagesComparator;
use stamp_generator::{Stamp, StampGenerator};
use evolution_algorithm::EvolutionAlgorithm;
use macroquad_canvas::Canvas2D;


const NUMBER_OF_ITERATIONS: u16 = 5;

#[macroquad::main("BasicShapes")]
async fn main() {
    let image_path: &str = "image.jpeg";
    let loaded_image: ImageBuffer<Rgba<u8>, Vec<u8>> = image::open(image_path)
            .expect("Could not find test-image")
            .into_rgba8();
    let image_size: (f32, f32) = (
        loaded_image.width() as f32,
        loaded_image.height() as f32
    );
    
    let gc: GraphicController = GraphicController::new(
        image_size.0,
        image_size.1
    ).await;

    let canvas = Canvas2D::new(image_size.0, image_size.1);
    set_camera(&canvas.camera);
    clear_background(WHITE);
    set_default_camera();
    next_frame().await;

    for _ in 0..NUMBER_OF_ITERATIONS {
        let loaded_image = image::open(image_path)
            .expect("Could not find test-image")
            .into_rgba8();
        let mut ea: EvolutionAlgorithm = EvolutionAlgorithm::new(
            StampGenerator::new(
                image_size.0,
                image_size.1
            ),
            ImagesComparator::new(
                loaded_image
            )
        );

        ea.eval_population(canvas.get_texture(), &gc).await;

        let stamp: &Stamp = ea.get_best_stamp();
        
        gc.draw(stamp, &canvas).await;
        gc.refresh_canvas(&canvas).await;

        // let second_image = gc.extract_image(&canvas);
        // let score: f64 = ic.compare_loaded_image_to(second_image);
        // println!("Simmilarity score: {}", score);
    }
}
