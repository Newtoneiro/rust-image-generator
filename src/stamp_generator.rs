use macroquad::color::Color;
use rand::Rng;
use std::f64::consts::PI;

const MIN_SIZE: f32 = 10.0;
const MAX_SIZE: f32 = 500.0;
const MIN_OPACITY: f32 = 0.2;
const MAX_OPACITY: f32 = 1.0;
const BORDER_OFFSET: f32 = 10.0;


pub struct Stamp {
    pub char: String,
    pub size: f32,
    pub color: Color,
    pub pos_x: f32,
    pub pos_y: f32,
    pub rotation: f32,
}

pub struct StampGenerator {
    max_width: f32,
    max_height: f32,
    rng: rand::prelude::ThreadRng,
}

impl StampGenerator {
    pub fn new(image_width: f32, image_height: f32) -> Self {
        Self {
            max_width: image_width,
            max_height: image_height,
            rng: rand::thread_rng(),
        }
    }

    fn generate_char(&mut self) -> String {
        let ascii_range: std::ops::RangeInclusive<u8> = b'!'..=b'~'; // ASCII values from 33 to 126
        let random_byte: u8 = self.rng.gen_range(ascii_range);
    
        (random_byte as char).to_string()
    }

    fn generate_size(&mut self) -> f32 {
        let size_range: std::ops::RangeInclusive<f32> = MIN_SIZE..=MAX_SIZE; // Possible font sizes from 8 to 64
        self.rng.gen_range(size_range)
    }

    fn generate_opacity(&mut self) -> f32 {
        self.rng.gen_range(MIN_OPACITY..=MAX_OPACITY)
    }

    fn generate_color(&mut self) -> Color {
        Color {
            r: self.rng.gen_range(0.0..=1.0),
            g: self.rng.gen_range(0.0..=1.0),
            b: self.rng.gen_range(0.0..=1.0),
            a: self.generate_opacity(),
        }
    }

    fn generate_position(&mut self) -> (f32, f32) {
        (
            self.rng.gen_range(0.0..self.max_width - BORDER_OFFSET),
            self.rng.gen_range(BORDER_OFFSET..self.max_height - BORDER_OFFSET),
        )
    }

    fn generate_rotation(&mut self) -> f32 {
        self.rng.gen_range(0.0..=2.0*PI) as f32
    }

    pub fn generate_stamp(&mut self) -> Stamp {
        let pos: (f32, f32) = self.generate_position();
        Stamp {
            char: self.generate_char(),
            size: self.generate_size(),
            color: self.generate_color(),
            pos_x: pos.0,
            pos_y: pos.1,
            rotation: self.generate_rotation(),
        }
    }
}