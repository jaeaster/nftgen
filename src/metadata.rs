use std::path::Path;

use crate::layer::Layer;
use serde::{Deserialize, Serialize};

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
    pub fn build<'a>(
        id: u32,
        description: &'a str,
        collection_name: &'a str,
        base_uri: &'a str,
        ordered_layers: &Vec<String>,
        layers: &Vec<&'a Layer>,
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
            format!("{}/{}.png", base_uri, id),
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

    pub fn update_image(&self) -> eyre::Result<()> {
        Ok(())
    }
}
