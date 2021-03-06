use std::{fmt::Display, fs::read_dir, path::Path};

use crate::{Layer, NftgenError};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

static IPFS_URI_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"ipfs://.*/").unwrap());

/// The high level metadata representation of the NFT collection.
/// - ```description```: Description of the NFT collection.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Attribute {
    trait_type: String,
    value: String,
}

impl Attribute {
    /// Returns a ```Attributes``` instance
    pub fn new(trait_type: String, value: String) -> Attribute {
        Attribute { trait_type, value }
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
            .zip(
                layers
                    .iter()
                    .map(|&l| l.name().expect("Layer name should be valid unicode")),
            )
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
    pub fn new<P: AsRef<Path> + ?Sized>(path: &'a P) -> MetadataWriter<'a> {
        MetadataWriter {
            path: path.as_ref(),
        }
    }

    pub fn write<P: AsRef<Path>>(
        &self,
        metadata: &Metadata,
        filename: P,
    ) -> Result<(), NftgenError> {
        let metadata_json = serde_json::to_string(&metadata)?;
        let metadata_file_path = self.path.join(filename);
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

    pub fn update_base_uri_for_all_images(&self, base_uri: &str) -> Result<(), NftgenError> {
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
                metadata_file_path.file_name().unwrap().to_str().unwrap(),
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod metadata_builder {
        use super::*;

        #[test]
        fn build() {
            let metadata = MetadataBuilder::build(
                3,
                "Great nft collection",
                "JustGreat",
                &["background", "face", "eyes"],
                &[
                    &Layer::new("red#2.png", 5),
                    &Layer::new("smile#5.png", 5),
                    &Layer::new("squint#5.png", 5),
                ],
            );

            assert_eq!(metadata.name, "JustGreat #3");
            assert_eq!(metadata.description, "Great nft collection");
            assert_eq!(metadata.image, "ipfs://placeholder/3.png");
            assert_eq!(
                metadata.attributes,
                vec![
                    Attribute {
                        trait_type: "background".to_string(),
                        value: "red".to_string(),
                    },
                    Attribute {
                        trait_type: "face".to_string(),
                        value: "smile".to_string(),
                    },
                    Attribute {
                        trait_type: "eyes".to_string(),
                        value: "squint".to_string(),
                    },
                ]
            );
        }
    }

    mod metadata_writer {
        use crate::nft::tests::fixture::Fixture;

        use super::*;

        #[test]
        fn write_and_update_base_uri() {
            let fixture = Fixture::blank("");
            let metadata_path = fixture.path.join("5.json");
            let writer = MetadataWriter::new(&fixture.path);

            let metadata = Metadata::new(
                "Some description",
                "Lame collection #5".to_string(),
                "ipfs://placeholder/5.png".to_string(),
                vec![
                    Attribute {
                        trait_type: "background".to_string(),
                        value: "red".to_string(),
                    },
                    Attribute {
                        trait_type: "face".to_string(),
                        value: "smile".to_string(),
                    },
                    Attribute {
                        trait_type: "eyes".to_string(),
                        value: "squint".to_string(),
                    },
                ],
            );

            writer.write(&metadata, "5.json").unwrap();
            let metadata_bytes = &std::fs::read(&metadata_path).unwrap();
            let updated_metadata: Metadata = serde_json::from_slice(metadata_bytes).unwrap();
            assert_eq!(metadata, updated_metadata);

            writer
                .update_base_uri_for_all_images("bussin-ipfs-cid")
                .unwrap();
            let metadata_bytes = &std::fs::read(&metadata_path).unwrap();
            let updated_metadata: Metadata = serde_json::from_slice(metadata_bytes).unwrap();
            assert_eq!(updated_metadata.image, "ipfs://bussin-ipfs-cid/5.png");
        }
    }
}
