use crate::layer::{Layer, LayerGroup};

use image::DynamicImage;

pub struct ImageBuilder {
    pub image: DynamicImage,
}

impl ImageBuilder {
    pub fn new(width: u32, heigth: u32) -> Self {
        let image = DynamicImage::new_rgba16(width, heigth);
        ImageBuilder { image }
    }

    pub fn add(&mut self, layer: &Layer) {
        image::imageops::overlay(&mut self.image, &layer.image, 0, 0)
    }

    pub fn build(layer_groups: &Vec<LayerGroup>) -> DynamicImage {
        let base = &layer_groups[0].pick().image;
        let width = base.width();
        let height = base.height();
        let mut builder = ImageBuilder::new(width, height);

        for layer_group in layer_groups.iter() {
            let layer = layer_group.pick();
            builder.add(&layer);
        }

        builder.image
    }
}
