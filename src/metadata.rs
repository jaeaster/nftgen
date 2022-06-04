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
