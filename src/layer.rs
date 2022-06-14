use std::cmp::Ordering;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

use color_eyre::eyre;
use eyre::Context;
use image::DynamicImage;
use log;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rayon::prelude::*;

/// Represents a value for single NFT layer
pub struct Layer {
    pub name: String,
    image_path: PathBuf,
    weight: u32,
}

impl Layer {
    pub fn new(name: &str, image_path: &Path, weight: u32) -> Self {
        Layer {
            name: name.to_string(),
            image_path: image_path.to_owned(),
            weight,
        }
    }

    pub fn get_image(&self) -> eyre::Result<DynamicImage> {
        image::open(&self.image_path).wrap_err(format!(
            "Failed to open image: {}",
            self.image_path.to_str().unwrap_or_default()
        ))
    }

    fn parse_layers_from_path<P: AsRef<Path>>(path: P) -> eyre::Result<Vec<Layer>> {
        let path = path.as_ref();
        path.read_dir()?
            .collect::<Result<Vec<DirEntry>, _>>()?
            .into_par_iter()
            .filter(|l| l.path().extension().unwrap_or_default() == "png")
            .map(|image_file| {
                log::debug!(
                    "Loading image from file: {}",
                    image_file.path().to_string_lossy()
                );
                Layer::try_from(image_file)
            })
            .collect::<Result<Vec<_>, _>>()
    }

    fn parse_weight_from_file_name(filename: &str) -> eyre::Result<u32> {
        let weight_str = filename.split_once('#').unwrap_or(("", "")).1;

        match weight_str.parse::<u32>() {
            Ok(weight) => Ok(weight),
            Err(_) => {
                log::warn!(
                    "Invalid weight for layer with filename: {}. Using default weight of 1",
                    filename
                );
                Ok(1)
            }
        }
    }
}

impl TryFrom<DirEntry> for Layer {
    type Error = eyre::Error;

    fn try_from(entry: DirEntry) -> eyre::Result<Self> {
        if let Some(name) = entry.path().file_stem() {
            match name.to_str() {
                Some(name) => Ok(Layer::new(
                    name,
                    &entry.path(),
                    Layer::parse_weight_from_file_name(name)?,
                )),
                None => eyre::bail!("Invalid layer name: {}", entry.path().display()),
            }
        } else {
            eyre::bail!("Invalid layer name: {}", entry.path().display());
        }
    }
}

/// Represents all of the values for a particular NFT layer group
/// e.g. Background, Foreground, etc.
pub struct LayerGroup {
    pub layer_type: String,
    layers: Vec<Layer>,
    order: u8,
}

impl LayerGroup {
    pub fn new<T: AsRef<str>, P: AsRef<Path>>(
        layer_path: P,
        layers_order: &[T],
    ) -> eyre::Result<Self> {
        let layer_path = layer_path.as_ref();
        let layers = Layer::parse_layers_from_path(layer_path)?;

        if let Some(layer_type_str) = layer_path.file_name() {
            let layer_type = layer_type_str.to_string_lossy().to_string();
            let order = LayerGroup::get_order(&layer_type, layers_order)?;
            Ok(LayerGroup {
                layer_type,
                layers,
                order,
            })
        } else {
            eyre::bail!("Invalid layer type: {}", layer_path.display());
        }
    }

    pub fn pick(&self) -> &Layer {
        let weights: Vec<_> = self.layers.iter().map(|l| l.weight).collect();
        let dist = WeightedIndex::new(&weights).unwrap();
        let mut rng = rand::thread_rng();

        &self.layers[dist.sample(&mut rng)]
    }

    fn get_order<T: AsRef<str>>(layer_type: &str, layers_order: &[T]) -> eyre::Result<u8> {
        match layers_order
            .iter()
            .position(|layer| layer.as_ref() == layer_type)
        {
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

pub fn get_layer_groups<T: AsRef<str>, P: AsRef<Path>>(
    layer_dir_root: P,
    layers_order: &[T],
) -> eyre::Result<Vec<LayerGroup>> {
    let layer_dir_root = layer_dir_root.as_ref();
    let layer_dirs = get_layer_dirs(layer_dir_root)?;
    for dir in &layer_dirs {
        log::info!("Found directory of layers: {}", dir.to_string_lossy());
    }

    layer_dirs
        .iter()
        .map(|layer_dir| LayerGroup::new(layer_dir.as_path(), layers_order))
        .collect()
}

fn get_layer_dirs<P: AsRef<Path>>(layer_dir_root: P) -> eyre::Result<Vec<PathBuf>> {
    let layer_dir_root = layer_dir_root.as_ref();
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
