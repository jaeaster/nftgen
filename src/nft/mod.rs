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
    pub mod fixture;
    use crate::nft::tests::fixture::Fixture;

    use super::*;
    use assert_str::assert_str_eq;

    #[test]
    fn get_layer_groups_works() {
        let layer_dirs = &["layer1", "layer2"];
        let fixture = Fixture::create_layers_dirs("minimal.png", layer_dirs);

        let layer_groups = get_layer_groups(&fixture.path, layer_dirs).unwrap();

        assert_eq!(layer_groups.len(), 2);
        assert!(matches!(
            layer_groups.iter().find(|lg| lg.layer_type == "layer1"),
            Some(_)
        ));
    }

    #[test]
    fn parse_layers_from_path_works() {
        let layer_dirs = &["background"];
        let fixture = Fixture::create_layers_dirs("minimal.png", layer_dirs);

        let layers = parse_layers_from_path(fixture.path.join("background")).unwrap();

        for i in 0..10 {
            let layer = layers.iter().find(|l| l.weight == i as u32).unwrap();
            assert_str_eq!(layer.name().unwrap(), format!("image{}", i));
        }
    }
}
