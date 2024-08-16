use ab_glyph::{FontRef, PxScale};
use image::{ImageBuffer, Rgb};
use imageproc::drawing::draw_text_mut;
use crate::stamp_generator::Stamp;


pub struct GraphicController {
    font: FontRef<'static>,
}


impl GraphicController {
    pub fn new() -> Self {
        let font = FontRef::try_from_slice(include_bytes!("DejaVuSans.ttf")).unwrap();
        let gc: GraphicController = Self { font };
        
        gc
    }

    pub fn draw(&self, canvas: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, stamp: &Stamp) {
        let scale = PxScale {
            x: stamp.size,
            y: stamp.size,
        };

        draw_text_mut(
            canvas,
            stamp.color,
            stamp.pos_x,
            stamp.pos_y,
            scale,
            &self.font,
            &stamp.char,
        );
    }
}
