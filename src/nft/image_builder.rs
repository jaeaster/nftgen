use crate::{Layer, LayerGroup};

use image::DynamicImage;

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

    pub fn add(&mut self, layer: &'a Layer) -> eyre::Result<()> {
        image::imageops::overlay(&mut self.image, &layer.get_image()?, 0, 0);
        self.layers.push(layer);
        Ok(())
    }

    pub fn build(layer_groups: &'a [LayerGroup]) -> eyre::Result<(DynamicImage, Vec<&'a Layer>)> {
        let base = layer_groups.get(0).unwrap().pick().get_image()?;
        let width = base.width();
        let height = base.height();
        log::debug!("Building image with width: {}, height: {}", width, height);
        let mut builder = ImageBuilder::new(width, height);

        for layer_group in layer_groups.iter() {
            let layer = layer_group.pick();
            log::debug!("Adding layer: {}", layer.name().unwrap_or_default());
            builder.add(layer)?;
            log::debug!("Added layer: {}", layer.name().unwrap_or_default());
        }

        Ok((builder.image, builder.layers))
    }
}
