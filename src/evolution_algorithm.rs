use macroquad::texture::Texture2D;
use macroquad_canvas::Canvas2D;
use ordered_float::OrderedFloat;
use crate::{
    graphic_controller::GraphicController,
    images_comparator::ImagesComparator,
    stamp_generator::{Stamp, StampGenerator}
};

const POPULATION_SIZE: u16 = 5;


struct Individual {
    stamp: Stamp,
    score: f64,
}

pub struct EvolutionAlgorithm {
    stamp_generator: StampGenerator,
    images_comparator: ImagesComparator,
    population: Vec<Individual>,
}

impl EvolutionAlgorithm {
    pub fn new(stamp_generator: StampGenerator, images_comparator: ImagesComparator) -> Self {
        let population: Vec<Individual> = Vec::new();
        
        let mut evolution_algorithm: EvolutionAlgorithm = EvolutionAlgorithm {
            stamp_generator,
            images_comparator,
            population
        };
        evolution_algorithm.init_population();

        evolution_algorithm
    }

    fn init_population(&mut self) -> () {
        self.population.clear();

        for _ in 0..POPULATION_SIZE {
            let stamp: Stamp = self.stamp_generator.generate_stamp();
            self.population.push(
                Individual { stamp: stamp, score: 0.0 }
            );
        }
    }

    pub async fn eval_population(
            &mut self,
            cur_texture: &Texture2D,
            graphic_controller: &GraphicController
        ) -> () {
        for individual in self.population.iter_mut() {
            let canvas: Canvas2D = graphic_controller.canvas_from_stamp_and_texture(&individual.stamp, cur_texture).await;
            individual.score = self.images_comparator.compare_loaded_image_to(graphic_controller.extract_image(&canvas));
        }
    }

    pub fn get_best_stamp(&mut self) -> &Stamp {
        &self.population.iter().max_by_key(|n| OrderedFloat(n.score)).unwrap().stamp
    }
}