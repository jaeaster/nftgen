use color_eyre::eyre;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

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

pub fn get_layer_groups(layer_dir_root: &Path) -> eyre::Result<Vec<LayerGroup>> {
    let layer_dirs = get_layer_dirs(layer_dir_root)?;

    layer_dirs
        .iter()
        .map(|layer_dir| LayerGroup::new(layer_dir.as_path()))
        .collect()
}

fn get_layer_dirs(layer_dir_root: &Path) -> eyre::Result<Vec<PathBuf>> {
    let layer_dirs: Vec<PathBuf> = match layer_dir_root.read_dir() {
        Ok(layers) => layers
            .collect::<Result<Vec<DirEntry>, _>>()?
            .into_iter()
            .map(|l| l.path())
            .filter(|l| l.is_dir())
            .collect(),
        Err(_) => eyre::bail!(
            "Failed to read layers directory: {}",
            layer_dir_root.to_string_lossy()
        ),
    };
    Ok(layer_dirs)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
