use color_eyre::eyre;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

use image::io::Reader as ImageReader;
use image::DynamicImage;

use rand::prelude::*;
use std::cmp::Ordering;

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
    order: u8,
}

impl LayerGroup {
    pub fn new(layer_path: &Path, layers_order: &Vec<String>) -> eyre::Result<Self> {
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

        let layer_type = layer_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let order = LayerGroup::get_order(&layer_type, layers_order)?;
        // let order = 0;
        Ok(LayerGroup {
            layer_type,
            layers,
            order,
        })
    }

    pub fn pick(&self) -> &Layer {
        let mut rng = rand::thread_rng();
        &self.layers[rng.gen_range(0..self.layers.len())]
    }

    fn get_order(layer_type: &str, layers_order: &Vec<String>) -> eyre::Result<u8> {
        match layers_order.iter().position(|layer| *layer == layer_type) {
            Some(order) => Ok(order as u8),
            None => eyre::bail!("Layer type {} not found in layers order", layer_type),
        }
    }
}

impl PartialOrd for LayerGroup {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.order.partial_cmp(&other.order)
    }
}

impl PartialEq for LayerGroup {
    fn eq(&self, other: &Self) -> bool {
        self.order == other.order
    }
}

pub fn get_layer_groups(
    layer_dir_root: &Path,
    layers_order: &Vec<String>,
) -> eyre::Result<Vec<LayerGroup>> {
    let layer_dirs = get_layer_dirs(layer_dir_root)?;

    layer_dirs
        .iter()
        .map(|layer_dir| LayerGroup::new(layer_dir.as_path(), layers_order))
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
