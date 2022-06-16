use std::cmp::Ordering;
use std::ffi::OsStr;
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
    image_path: PathBuf,
    weight: u32,
}

impl Layer {
    pub fn new(image_path: &Path, weight: u32) -> Self {
        Layer {
            image_path: image_path.to_owned(),
            weight,
        }
    }

    /// Returns none if the filename is not valid unicode
    pub fn name(&self) -> Option<&str> {
        self.image_path.file_stem()?.to_str()?.split('#').next()
    }

    pub fn get_image(&self) -> eyre::Result<DynamicImage> {
        image::open(&self.image_path).wrap_err(format!(
            "Failed to open image: {}",
            self.image_path.to_str().unwrap_or_default()
        ))
    }

    /// Parses layer files within a directory into Layer structs
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

    /// Parses weight from file stem of image file
    /// `filestem` - A stem of a file i.e. filename without extension e.g. red#5
    fn parse_weight_from_file_stem(filestem: &str) -> eyre::Result<u32> {
        let weight_str = filestem.split_once('#').unwrap_or(("", "")).1;

        match weight_str.parse::<u32>() {
            Ok(weight) => Ok(weight),
            Err(_) => {
                log::warn!(
                    "Invalid weight for layer with filestem: {}. Using default weight of 1",
                    filestem
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
                    &entry.path(),
                    Layer::parse_weight_from_file_stem(name)?,
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
    use super::*;
    use assert_str::assert_str_eq;
    use std::env::temp_dir;
    static PNG: [u8; 67] = [
        0x89, 0x50, 0x4e, 0x47, 0xd, 0xa, 0x1a, 0xa, 0x0, 0x0, 0x0, 0xd, 0x49, 0x48, 0x44, 0x52,
        0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x1, 0x8, 0x6, 0x0, 0x0, 0x0, 0x1f, 0x15, 0xc4, 0x89,
        0x0, 0x0, 0x0, 0xa, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9c, 0x63, 0x0, 0x1, 0x0, 0x0, 0x5, 0x0,
        0x1, 0xd, 0xa, 0x2d, 0xb4, 0x0, 0x0, 0x0, 0x0, 0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60,
        0x82,
    ];

    static INVALID_PNG: [u8; 0] = [];

    #[test]
    fn layer_name_works() {
        let image_path = std::path::Path::new("layers/background/red#5.png");
        let layer = Layer::new(image_path, 5);
        assert_str_eq!(layer.name().unwrap(), "red");
    }

    #[test]
    fn layer_name_works_without_delimiter() {
        let image_path = std::path::Path::new("layers/background/red.png");
        let layer = Layer::new(image_path, 5);
        assert_str_eq!(layer.name().unwrap(), "red");
    }

    #[test]
    fn layer_name_returns_none_if_not_utf8() {
        use std::os::unix::ffi::OsStrExt;
        let source = [0x66, 0x6f, 0x80, 0x6f];
        let os_str = OsStr::from_bytes(&source[..]);
        let image_path = std::path::Path::new(os_str);

        let layer = Layer::new(image_path, 5);
        assert_eq!(layer.name(), None);
    }

    #[test]
    fn layer_get_image_works() {
        let tmp = temp_dir();
        let image_path = tmp.join("image.png");
        std::fs::write(&image_path, PNG).expect("Write to tmp should work in test");

        let layer = Layer::new(&image_path, 5);
        let image = layer.get_image();
        assert!(image.is_ok());
    }

    #[test]
    fn layer_get_image_returns_err_for_invalid_png() {
        let tmp = temp_dir();
        let image_path = tmp.join("image_err.png");
        std::fs::write(&image_path, INVALID_PNG).expect("Write to tmp should work in test");

        let layer = Layer::new(&image_path, 5);
        let image = layer.get_image();
        assert!(image.is_err());
    }

    #[test]
    fn parse_weight_from_filestem_works() {
        assert_eq!(Layer::parse_weight_from_file_stem("beauty#10").unwrap(), 10)
    }

    #[test]
    fn parse_weight_from_filestem_returns_1_if_invalid_filestem() {
        assert_eq!(Layer::parse_weight_from_file_stem("beauty").unwrap(), 1)
    }

    #[test]
    fn parse_layers_from_path_works() {
        let tmp = temp_dir().join("layers");
        std::fs::remove_dir_all(&tmp).expect("Remove tmp layers dir should work in test");
        std::fs::create_dir(&tmp).expect("Create tmp layers dir should work in test");
        for i in 0..10 {
            let image_path = tmp.join(format!("image{}#{}.png", i, i));
            std::fs::write(&image_path, PNG).expect("Write to tmp should work in test");
        }

        let layers = Layer::parse_layers_from_path(tmp).unwrap();

        for i in 0..10 {
            let layer = layers.iter().find(|l| l.weight == i as u32).unwrap();
            assert_str_eq!(layer.name().unwrap(), format!("image{}", i));
        }
    }
}
