use rayon::prelude::*;
use std::{
    fs::DirEntry,
    path::{Path, PathBuf},
};

mod image_builder;
mod layer;
mod layer_group;
mod metadata;

pub use image_builder::*;
pub use layer::*;
pub use layer_group::*;
pub use metadata::*;

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

    #[test]
    fn parse_layers_from_path_works() {
        let tmp = temp_dir().join("layers");
        std::fs::remove_dir_all(&tmp).expect("Remove tmp layers dir should work in test");
        std::fs::create_dir(&tmp).expect("Create tmp layers dir should work in test");
        for i in 0..10 {
            let image_path = tmp.join(format!("image{}#{}.png", i, i));
            std::fs::write(&image_path, PNG).expect("Write to tmp should work in test");
        }

        let layers = parse_layers_from_path(tmp).unwrap();

        for i in 0..10 {
            let layer = layers.iter().find(|l| l.weight == i as u32).unwrap();
            assert_str_eq!(layer.name().unwrap(), format!("image{}", i));
        }
    }
}
