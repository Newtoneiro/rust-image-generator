use image::Rgb;
use rand::Rng;

const MIN_SIZE: f32 = 10.0;
const MAX_SIZE: f32 = 500.0;

#[derive(Clone)]
#[derive(Debug)]
pub struct Stamp {
    pub char: String,
    pub size: f32,
    pub color: Rgb<u8>,
    pub pos_x: i32,
    pub pos_y: i32,
}

pub struct StampGenerator {
    max_width: i32,
    max_height: i32,
    rng: rand::prelude::ThreadRng,
}

impl StampGenerator {
    pub fn new(canvas_width: i32, canvas_height: i32) -> Self {
        Self {
            max_width: canvas_width,
            max_height: canvas_height,
            rng: rand::thread_rng(),
        }
    }

    pub fn generate_char(&mut self) -> String {
        let ascii_range: std::ops::RangeInclusive<u8> = b'!'..=b'~'; // ASCII values from 33 to 126
        let random_byte: u8 = self.rng.gen_range(ascii_range);
    
        (random_byte as char).to_string()
    }

    pub fn generate_size(&mut self) -> f32 {
        let size_range: std::ops::RangeInclusive<f32> = MIN_SIZE..=MAX_SIZE; // Possible font sizes from 8 to 64
        self.rng.gen_range(size_range)
    }

    pub fn generate_color(&mut self) -> Rgb<u8> {
        Rgb([
            self.rng.gen_range(0..=255),
            self.rng.gen_range(0..=255),
            self.rng.gen_range(0..=255),
        ])
    }

    pub fn generate_position(&mut self) -> (i32, i32) {
        (
            self.rng.gen_range(0..self.max_width),
            self.rng.gen_range(0..self.max_height),
        )
    }

    pub fn generate_stamp(&mut self) -> Stamp {
        let (x, y ) = self.generate_position();
        Stamp {
            char: self.generate_char(),
            size: self.generate_size(),
            color: self.generate_color(),
            pos_x: x,
            pos_y: y,
        }
    }
}