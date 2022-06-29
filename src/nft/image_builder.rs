use crate::{Image, Layer, LayerGroup, NftgenError};

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

    pub fn add(&mut self, layer: &'a Layer) {
        self.layers.push(layer);
    }

    pub fn build(layer_groups: &'a [LayerGroup]) -> Result<(Image, Vec<&'a Layer>), NftgenError> {
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
            builder.add(layer);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nft::tests::fixture::Fixture;

    #[test]
    fn add() {
        let layers = vec![
            Layer::new("bkg.png", 1),
            Layer::new("face.png", 1),
            Layer::new("eyes.png", 1),
        ];
        let mut builder = ImageBuilder::new(Image::new(vec![0], 1, 1, 1));
        for layer in layers.iter() {
            builder.add(layer);
        }

        for (expected_layer, actual_layer) in layers.iter().zip(builder.layers) {
            assert_eq!(expected_layer, actual_layer);
        }
    }

    mod build {
        use super::*;
        use crate::get_layer_groups;

        #[test]
        fn build() {
            let layer_dirs = &["layer1", "layer2"];
            let fixture = Fixture::create_layers_dirs("minimal.png", layer_dirs);

            let layer_groups = get_layer_groups(&fixture.path, layer_dirs).unwrap();

            let (image, layers) = ImageBuilder::build(&layer_groups).unwrap();
            let expected = Image::read(fixture.path.join("layer1/image1#1.png")).unwrap();
            assert_eq!(expected, image);
            assert_eq!(layers.len(), 2);
        }
    }
}
