use img_hash::{HasherConfig, ImageHash, Hasher};
use image::RgbImage;

pub struct ImagesComparator {
    hasher: Hasher,
    loaded_image_hash: ImageHash,
}

impl ImagesComparator {
    pub fn new(loaded_image: RgbImage) -> Self {
        let hasher = HasherConfig::new()
            .hash_alg(img_hash::HashAlg::Mean)
            .hash_size(256, 256)
            .to_hasher();
        let loaded_image_hash = hasher.hash_image(&loaded_image);
        Self { hasher, loaded_image_hash }
    }

    pub fn compare_loaded_image_to(&self, second_image: &RgbImage) -> f64 {
        let hash2 = self.hasher.hash_image(second_image);
        let distance = self.loaded_image_hash.dist(&hash2) as f64;
        distance
    }
}