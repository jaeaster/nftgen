use crate::{Image, Layer, LayerGroup};

pub struct ImageBuilder<'a> {
    pub image: Image,
    pub layers: Vec<&'a Layer>,
}

impl<'a> ImageBuilder<'a> {
    pub fn new(image: Image) -> Self {
        ImageBuilder {
            image,
            layers: vec![],
        }
    }

    pub fn add(&mut self, layer: &'a Layer) -> eyre::Result<()> {
        self.layers.push(layer);
        Ok(())
    }

    pub fn build(layer_groups: &'a [LayerGroup]) -> eyre::Result<(Image, Vec<&'a Layer>)> {
        let base = layer_groups.get(0).unwrap().pick().get_image()?;
        log::debug!(
            "Building image with width: {}, height: {}",
            base.width,
            base.height
        );
        let mut builder = ImageBuilder::new(base);

        for layer_group in layer_groups.iter() {
            let layer = layer_group.pick();
            log::debug!("Adding layer: {}", layer.name().unwrap_or_default());
            builder.add(layer)?;
            log::debug!("Added layer: {}", layer.name().unwrap_or_default());
        }
        builder.image.stack(
            builder
                .layers
                .iter()
                .map(|layer| layer.get_image())
                .collect::<Result<Vec<_>, _>>()?
                .as_slice(),
        );

        Ok((builder.image, builder.layers))
    }
}
