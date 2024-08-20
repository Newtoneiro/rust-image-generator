use img_hash::image::{ImageBuffer, Rgb};
use imageproc::drawing::draw_text_mut;
use crate::stamp_generator::Stamp;
use rusttype::{Font, Scale};


pub struct GraphicController {
    font: Font<'static>,
}


impl GraphicController {
    pub fn new() -> Self {
        let font = Font::try_from_bytes(include_bytes!("DejaVuSans.ttf")).unwrap();
        let gc: GraphicController = Self { font };
        
        gc
    }

    pub fn draw(&self, canvas: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, stamp: &Stamp) {
        let scale = Scale {
            x: stamp.size,
            y: stamp.size,
        };

        draw_text_mut(
            canvas,
            stamp.color,
            stamp.pos_x as u32,
            stamp.pos_y as u32,
            scale,
            &self.font,
            &stamp.char,
        );
    }
}
