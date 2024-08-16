pub struct ImagesComparator {
    loaded_image: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
}

impl ImagesComparator {
    pub fn new(loaded_image: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>) -> Self {
        Self { loaded_image }
    }

    pub fn compare_loaded_image_to(&self, second_image: &image::ImageBuffer<image::Rgb<u8>, Vec<u8>>) -> f64 {
        let result: image_compare::Similarity = image_compare::rgb_hybrid_compare(
            (&self.loaded_image).into(),
            second_image.into(),
        ).expect("Images had different dimensions");
        
        result.score
    }
}