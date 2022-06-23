use std::cmp::Ordering;
use std::path::Path;

use rand::distributions::WeightedIndex;
use rand::prelude::*;

use crate::nft::parse_layers_from_path;
use crate::{Layer, NftgenError};

/// Represents all of the values for a particular NFT layer group
/// e.g. Background, Foreground, etc.
#[derive(Debug)]
pub struct LayerGroup {
    pub layer_type: String,
    layers: Vec<Layer>,
    order: u8,
}

impl LayerGroup {
    pub fn new<T: AsRef<str>, P: AsRef<Path>>(
        layer_path: P,
        layers_order: &[T],
    ) -> Result<Self, NftgenError> {
        let layer_path = layer_path.as_ref();
        let layers = parse_layers_from_path(layer_path)?;

        if let Some(layer_type_str) = layer_path.file_name() {
            let layer_type = layer_type_str.to_string_lossy().to_string();
            let order = LayerGroup::get_order(&layer_type, layers_order)?;
            Ok(LayerGroup {
                layer_type,
                layers,
                order,
            })
        } else {
            Err(NftgenError::InvalidLayerPath(layer_path.to_owned()))
        }
    }

    pub fn pick(&self) -> &Layer {
        let weights: Vec<_> = self.layers.iter().map(|l| l.weight).collect();
        let dist = WeightedIndex::new(&weights).unwrap();
        let mut rng = rand::thread_rng();

        &self.layers[dist.sample(&mut rng)]
    }

    fn get_order<T: AsRef<str>>(layer_type: &str, layers_order: &[T]) -> Result<u8, NftgenError> {
        match layers_order
            .iter()
            .position(|layer| layer.as_ref() == layer_type)
        {
            Some(order) => Ok(order as u8),
            None => Err(NftgenError::UnknownLayer(layer_type.to_string())),
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
