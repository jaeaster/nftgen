use crate::layer::{Layer, LayerGroup};

use image::DynamicImage;
use log;

pub struct ImageBuilder<'a> {
    pub image: DynamicImage,
    pub layers: Vec<&'a Layer>,
}

impl<'a> ImageBuilder<'a> {
    pub fn new(width: u32, heigth: u32) -> Self {
        let image = DynamicImage::new_rgba16(width, heigth);
        ImageBuilder {
            image,
            layers: vec![],
        }
    }

    pub fn add(&mut self, layer: &'a Layer) {
        image::imageops::overlay(&mut self.image, &layer.image, 0, 0);
        self.layers.push(layer);
    }

    pub fn build(layer_groups: &'a Vec<LayerGroup>) -> (DynamicImage, Vec<&'a Layer>) {
        let base = &layer_groups[0].pick().image;
        let width = base.width();
        let height = base.height();
        log::debug!("Building image with width: {}, height: {}", width, height);
        let mut builder = ImageBuilder::new(width, height);

        for layer_group in layer_groups.iter() {
            let layer = layer_group.pick();
            log::debug!("Adding layer: {}", layer.name);
            builder.add(&layer);
            log::debug!("Added layer: {}", layer.name);
        }

        (builder.image, builder.layers)
    }
}
