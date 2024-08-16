use image::{ImageBuffer, Rgb};
use ordered_float::OrderedFloat;
use crate::{
    graphic_controller::GraphicController,
    images_comparator::ImagesComparator,
    stamp_generator::{Stamp, StampGenerator}
};
use rand::{seq::SliceRandom, Rng};


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

    pub fn eval_population(
        &mut self,
        cur_image: &ImageBuffer<Rgb<u8>, Vec<u8>>,
        graphic_controller: &GraphicController,
        images_comparator: &ImagesComparator,
    ) {
        for individual in self.population.iter() {
            EvolutionAlgorithm::eval_individual(
                &individual.stamp,
                &mut cur_image.clone(),
                &graphic_controller,
                &images_comparator,
            );
        };
    }

    fn eval_individual(
        stamp: &Stamp,
        image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
        graphic_controller: &GraphicController,
        images_comparator: &ImagesComparator
    ) -> f64 {
        graphic_controller.draw(image, stamp);

        images_comparator.compare_loaded_image_to(&image)
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
            color: Rgb([
                rng.gen_range(parent1.stamp.color[0].min(parent2.stamp.color[0])..=parent1.stamp.color[0].max(parent2.stamp.color[0])),
                rng.gen_range(parent1.stamp.color[1].min(parent2.stamp.color[1])..=parent1.stamp.color[1].max(parent2.stamp.color[1])),
                rng.gen_range(parent1.stamp.color[2].min(parent2.stamp.color[2])..=parent1.stamp.color[2].max(parent2.stamp.color[2])),
            ]),

            pos_x: (parent1.stamp.pos_x + parent2.stamp.pos_x) / 2,
            pos_y: (parent1.stamp.pos_y + parent2.stamp.pos_y) / 2,
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
            let rand_pos: (i32, i32) = stamp_generator.generate_position();
            individual.stamp.pos_x = rand_pos.0; // Random position in a reasonable range
            individual.stamp.pos_y = rand_pos.1;
        }
    }

    fn get_best_stamp(&mut self) -> &Stamp {
        &self.population.iter().max_by_key(|n| OrderedFloat(n.score)).unwrap().stamp
    }

    pub fn run(
        &mut self,
        starting_image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
        graphic_controller: &GraphicController,
        images_comparator: &ImagesComparator,
        stamp_generator: &mut StampGenerator
    ) -> &Stamp {
        self.init_population(stamp_generator);

        for epoch in 0..EPOCHS {
            println!("EPOCH: {:?}/{:?}", epoch, EPOCHS);
            println!("Eval population...");
            self.eval_population(
                starting_image,
                graphic_controller,
                images_comparator
            );
            println!("Make new generation...");
            self.make_new_generation(stamp_generator);
        }

        self.get_best_stamp()
    }
}