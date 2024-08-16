mod graphic_controller;
mod images_comparator;
mod stamp_generator;
mod evolution_algorithm;


use evolution_algorithm::EvolutionAlgorithm;
use graphic_controller::GraphicController;
use image::{ImageBuffer, RgbImage, Rgb};
use images_comparator::ImagesComparator;
use stamp_generator::{StampGenerator, Stamp};


const NUMBER_OF_ITERATIONS: u16 = 1000;
const IMAGE_PATH: &str = "image.jpg";
const OUTPUT_PATH: &str = "./output.png";

struct ImageSize {
    width: u32,
    height: u32,
}

fn main() {
    let loaded_image: ImageBuffer<Rgb<u8>, Vec<u8>> = image::open(IMAGE_PATH)
        .expect("Could not find test-image")
        .into_rgb8();
    let image_size: ImageSize = ImageSize {
        width: loaded_image.width(),
        height: loaded_image.height(),
    };
    let ic: ImagesComparator = ImagesComparator::new(loaded_image);

    let mut sg:  StampGenerator = StampGenerator::new(
        image_size.width as i32,
        image_size.height as i32
    );

    let mut image = RgbImage::new(image_size.width, image_size.height);
    image.fill(255);

    let mut ea: EvolutionAlgorithm = EvolutionAlgorithm::new();
    let gc: GraphicController = GraphicController::new();
    
    for _ in 0..=NUMBER_OF_ITERATIONS {
        let stamp: &Stamp = ea.run(&mut image, &gc, &ic, &mut sg);
    
        gc.draw(&mut image, &stamp);
        image.save(OUTPUT_PATH).unwrap();
    }
}
