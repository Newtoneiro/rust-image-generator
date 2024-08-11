use macroquad::{color::Color, texture::Texture2D};
use macroquad_canvas::Canvas2D;
use ordered_float::OrderedFloat;
use crate::{
    graphic_controller::GraphicController, images_comparator::{self, ImagesComparator}, stamp_generator::{self, Stamp, StampGenerator}
};
use rand::{seq::SliceRandom, Rng};
use futures::stream::FuturesUnordered;
use futures::stream::StreamExt;

const POPULATION_SIZE: u16 = 20;
const PROMOTION_RATIO: f32 = 0.25;
const INDIVIDUAL_MUT_PROB: f64 = 0.9;
const ATTRIBUTE_MUT_PROB: f64 = 0.2;
const EPOCHS: u16 = 20;


struct Individual {
    stamp: Stamp,
    score: f64,
}

pub struct EvolutionAlgorithm {
    population: Vec<Individual>,
}

impl EvolutionAlgorithm {
    pub fn new() -> Self {
        let population: Vec<Individual> = Vec::new();
        let evolution_algorithm: EvolutionAlgorithm = EvolutionAlgorithm {
            population,
        };

        evolution_algorithm
    }

    fn init_population(&mut self, stamp_generator: &mut StampGenerator) -> () {
        self.population.clear();

        for _ in 0..POPULATION_SIZE {
            let stamp: Stamp = stamp_generator.generate_stamp();
            self.population.push(
                Individual { stamp: stamp, score: 0.0 }
            );
        }
    }

    pub async fn eval_population(
            &mut self,
            cur_texture: &Texture2D,
            graphic_controller: &GraphicController,
            images_comparator: &ImagesComparator
        ) -> () {
        let mut futures = FuturesUnordered::new();

        for individual in self.population.iter_mut() {
            let graphic_controller_clone = graphic_controller.clone();
            let cur_texture_clone = cur_texture.clone();
            let images_comparator_clone = images_comparator.clone();
        
            let future = async move {
                let canvas: Canvas2D = graphic_controller_clone.canvas_from_stamp_and_texture(&individual.stamp, &cur_texture_clone).await;
                let score = images_comparator_clone.compare_loaded_image_to(graphic_controller_clone.extract_image(&canvas));
                (individual, score)
            };
        
            futures.push(future);
        }

        while let Some((individual, score)) = futures.next().await {
            individual.score = score;
        }
    }

    pub fn make_new_generation(&mut self, stamp_generator: &mut StampGenerator) {
        self.population.sort_by_key(|individual| OrderedFloat(individual.score));
        let num_to_keep = (PROMOTION_RATIO * POPULATION_SIZE as f32) as usize;
        self.population.truncate(num_to_keep);

        let mut new_population: Vec<Individual> = Vec::new();

        // Cross breeding
        println!("- Cross breeding...");
        while new_population.len() < POPULATION_SIZE as usize {
            new_population.push(self.create_offspring(&self.population[..num_to_keep]))
        }

        // Mutation
        println!("- Mutation...");
        for mut individual in new_population.iter_mut() {
            self.mutate_individual(&mut individual, stamp_generator);
        }

        // Replace population
        println!("- Replace population...");
        self.population = new_population;
    }

    fn create_offspring(&self, parents: &[Individual]) -> Individual {
        let mut rng = rand::thread_rng();

        // Step 1: Select two random parents
        println!("-- Select 2 parents...");
        let selected_parents: Vec<&Individual> = parents.choose_multiple(&mut rng, 2).collect();
        let parent1 = selected_parents[0];
        let parent2 = selected_parents[1];

        // Step 2: Combine their attributes to create an offspring
        println!("-- Combine 2 parents...");
        let offspring_stamp = Stamp {
            char: if rng.gen_bool(0.5) { parent1.stamp.char.clone() } else { parent2.stamp.char.clone() },
            size: (parent1.stamp.size + parent2.stamp.size) / 2.0, // Average size
            color: Color {
                r: rng.gen_range(parent1.stamp.color.r.min(parent2.stamp.color.r)..=parent1.stamp.color.r.max(parent2.stamp.color.r)),
                g: rng.gen_range(parent1.stamp.color.g.min(parent2.stamp.color.g)..=parent1.stamp.color.g.max(parent2.stamp.color.g)),
                b: rng.gen_range(parent1.stamp.color.b.min(parent2.stamp.color.b)..=parent1.stamp.color.b.max(parent2.stamp.color.b)),
                a: rng.gen_range(parent1.stamp.color.a.min(parent2.stamp.color.a)..=parent1.stamp.color.a.max(parent2.stamp.color.a)),
            },
            pos_x: (parent1.stamp.pos_x + parent2.stamp.pos_x) / 2.0,
            pos_y: (parent1.stamp.pos_y + parent2.stamp.pos_y) / 2.0,
            rotation: (parent1.stamp.rotation + parent2.stamp.rotation) / 2.0,
        };

        // Step 3: Create a new Individual with the offspring's stamp and default or calculated score
        println!("-- Create offspring...");
        Individual {
            stamp: offspring_stamp,
            score: 0.0,
        }
    }

    fn mutate_individual(&mut self, individual: &mut Individual, stamp_generator: &mut StampGenerator) -> () {
        let mut rng = rand::thread_rng();

        if rng.gen_bool(1.0 - INDIVIDUAL_MUT_PROB) {
            ()
        }

        // Mutate char
        if rng.gen_bool(ATTRIBUTE_MUT_PROB) {
            individual.stamp.char = stamp_generator.generate_char(); // Function to generate a random char
        }

        // Mutate size
        if rng.gen_bool(ATTRIBUTE_MUT_PROB) {
            individual.stamp.size = stamp_generator.generate_size(); // Random size in a reasonable range
        }

        // Mutate color
        if rng.gen_bool(ATTRIBUTE_MUT_PROB) {
            individual.stamp.color = stamp_generator.generate_color();
        }

        // Mutate position
        if rng.gen_bool(ATTRIBUTE_MUT_PROB) {
            let rand_pos: (f32, f32) = stamp_generator.generate_position();
            individual.stamp.pos_x = rand_pos.0; // Random position in a reasonable range
            individual.stamp.pos_y = rand_pos.1;
        }

        // Mutate rotation
        if rng.gen_bool(ATTRIBUTE_MUT_PROB) {
            individual.stamp.rotation = stamp_generator.generate_rotation(); // Random rotation in degrees
        }
    }

    fn get_best_stamp(&mut self) -> &Stamp {
        &self.population.iter().max_by_key(|n| OrderedFloat(n.score)).unwrap().stamp
    }

    pub async fn run(
        &mut self,
        starting_texture: &Texture2D,
        graphic_controller: &GraphicController,
        images_comparator: &ImagesComparator,
        stamp_generator: &mut StampGenerator
    ) -> &Stamp {
        self.init_population(stamp_generator);

        for epoch in 0..EPOCHS {
            println!("EPOCH: {:?}/{:?}", epoch, EPOCHS);
            println!("Eval population...");
            self.eval_population(
                starting_texture,
                graphic_controller,
                images_comparator
            ).await;
            println!("Make new generation...");
            self.make_new_generation(stamp_generator);
        }

        self.get_best_stamp()
    }
}