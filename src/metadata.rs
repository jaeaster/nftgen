use std::{fmt::Display, fs::read_dir, path::Path};

use crate::layer::Layer;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

static IPFS_URI_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"ipfs://.*/").unwrap());

/// The high level metadata representation of the NFT collection.
/// - ```description```: Description of the NFT collection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata<'a> {
    pub description: &'a str,
    pub name: String,
    pub image: String,
    pub attributes: Vec<Attribute>,
}

impl<'a> Metadata<'a> {
    pub fn new(
        description: &'a str,
        name: String,
        image: String,
        attributes: Vec<Attribute>,
    ) -> Metadata {
        Metadata {
            description,
            name,
            image,
            attributes,
        }
    }
}

/// Attributes related to the NFT. This is automatically generated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    trait_type: String,
    value: String,
}

impl Attribute {
    /// Returns a ```Attributes``` instance
    pub fn new(trait_type: String, value: String) -> Attribute {
        Attribute {
            trait_type: trait_type,
            value: value,
        }
    }
}

pub struct MetadataBuilder {}

impl MetadataBuilder {
    pub fn build<'a, T: AsRef<str> + Display>(
        id: u32,
        description: &'a str,
        collection_name: &str,
        ordered_layers: &[T],
        layers: &[&Layer],
    ) -> Metadata<'a> {
        let attributes: Vec<Attribute> = ordered_layers
            .iter()
            .zip(layers.iter().map(|&l| l.name.as_str()))
            .map(|(layer_type, layer_name)| {
                Attribute::new(layer_type.to_string(), layer_name.to_string())
            })
            .collect();

        Metadata::new(
            description,
            format!("{} #{}", collection_name, id),
            format!("ipfs://placeholder/{}.png", id),
            attributes,
        )
    }
}

pub struct MetadataWriter<'a> {
    path: &'a Path,
}

impl<'a> MetadataWriter<'a> {
    pub fn new(path: &'a Path) -> MetadataWriter<'a> {
        MetadataWriter { path }
    }

    pub fn write(&self, metadata: &Metadata, filename: &str) -> eyre::Result<()> {
        let metadata_json = serde_json::to_string(&metadata)?;
        let metadata_file_path = self.path.join(format!("{}", filename));
        log::debug!(
            "Writing metadata to file: {}",
            metadata_file_path.to_string_lossy()
        );
        std::fs::write(&metadata_file_path, metadata_json)?;
        log::debug!(
            "Saved metadata to file: {}",
            metadata_file_path.to_string_lossy()
        );
        Ok(())
    }

    pub fn update_base_uri_for_all_images(&self, base_uri: &str) -> eyre::Result<()> {
        log::info!("Updating base_uri for all images with: {}", base_uri);

        let entries = read_dir(self.path)?
            .map(|entry| entry.unwrap())
            .map(|entry| entry.path());

        for metadata_file_path in entries {
            let metadata_json = std::fs::read_to_string(&metadata_file_path)?;
            let mut metadata: Metadata = serde_json::from_str(&metadata_json)?;
            let new_image_uri =
                IPFS_URI_REGEX.replace_all(&metadata.image, &format!("ipfs://{}/", base_uri));
            metadata.image = new_image_uri.to_string();
            self.write(
                &metadata,
                &metadata_file_path.file_name().unwrap().to_str().unwrap(),
            )?;
        }
        Ok(())
    }
}
