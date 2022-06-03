use color_eyre::eyre;
use std::{fs::DirEntry, path::Path};

use image::io::Reader as ImageReader;
use image::DynamicImage;

use rand::prelude::*;

/// Represents a value for single NFT layer
pub struct Layer {
    pub name: String,
    pub image: DynamicImage,
}

/// Represents all of the values for a particular NFT layer group
/// e.g. Background, Foreground, etc.
pub struct LayerGroup {
    pub layer_type: String,
    pub layers: Vec<Layer>,
}

impl LayerGroup {
    pub fn new(layer_path: &Path) -> eyre::Result<Self> {
        let layers = layer_path
            .read_dir()?
            .collect::<Result<Vec<DirEntry>, _>>()?
            .into_iter()
            .filter(|l| l.path().extension().unwrap_or_default() == "png")
            .map(|i| match ImageReader::open(i.path()) {
                Ok(reader) => (
                    i.file_name()
                        .to_string_lossy()
                        .split_once(".")
                        .unwrap()
                        .0
                        .to_string(),
                    reader,
                ),
                Err(_) => panic!("Failed to open image file"),
            })
            .map(|(name, r)| match r.decode() {
                Ok(image) => Layer { name, image },
                Err(_) => panic!("Failed to decode image"),
            })
            .collect();

        Ok(LayerGroup {
            layer_type: layer_path.to_string_lossy().to_string(),
            layers,
        })
    }

    pub fn pick(&self) -> &Layer {
        let mut rng = rand::thread_rng();
        &self.layers[rng.gen_range(0..self.layers.len())]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
