use std::path::PathBuf;
use std::{fs, sync::atomic::AtomicU32};

use clap::Parser;
use rayon::prelude::*;

use crate::cmd::Cmd;
use crate::metadata::MetadataWriter;
use crate::{image_builder::ImageBuilder, layer::get_layer_groups, metadata::MetadataBuilder};

#[derive(Debug, Clone, Parser)]
pub struct GenerateArgs {
    /// Number of NFTs to generate
    #[clap(short, long)]
    pub num: usize,

    /// path to root directory of NFT layers
    #[clap(short, long, default_value="./layers", value_hint = clap::ValueHint::DirPath)]
    pub layers_path: PathBuf,

    /// path to root directory of NFT layers
    #[clap(short, long, default_value="./nftgen-output", value_hint = clap::ValueHint::DirPath)]
    pub output_path: PathBuf,

    /// Order of NFT layers from back to front
    #[clap(
        long,
        multiple_values(true),
        use_value_delimiter(true),
        require_value_delimiter(true)
    )]
    pub layers_order: Vec<String>,

    /// Name of the collection
    #[clap(short, long)]
    pub collection_name: String,

    /// Description for the collection
    #[clap(short, long)]
    pub description: String,

    /// Base URI for assets in the collection
    #[clap(short, long)]
    pub base_uri: String,
}

impl Cmd for GenerateArgs {
    type Output = ();

    fn run(self) -> eyre::Result<Self::Output> {
        let images_path = self.output_path.as_path().join("images");
        let metadata_path = self.output_path.as_path().join("metadata");

        fs::create_dir_all(images_path.as_path())?;
        fs::create_dir_all(metadata_path.as_path())?;

        log::debug!("Parsing layer groups");
        let mut layer_groups = get_layer_groups(&self.layers_path, &self.layers_order)?;
        log::debug!(
            "Sorting layer groups according to order: {}",
            self.layers_order.join(", ")
        );
        layer_groups.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let counter = AtomicU32::new(0);
        log::debug!("Creating Images and Metadata");
        let results: eyre::Result<Vec<()>> = (0..self.num)
            .into_par_iter()
            .map(|n| {
                let image_file_path = images_path.as_path().join(format!("{}.png", n));

                let (nft, layers) = ImageBuilder::build(&layer_groups);
                let metadata = MetadataBuilder::build(
                    n as u32,
                    &self.description,
                    &self.collection_name,
                    &self.base_uri,
                    &self.layers_order,
                    &layers,
                );

                log::debug!(
                    "Writing image to file: {}",
                    image_file_path.to_string_lossy()
                );
                match nft.save_with_format(&image_file_path, image::ImageFormat::Png) {
                    Ok(_) => {
                        log::debug!("Saved image to file: {}", image_file_path.to_string_lossy())
                    }
                    Err(e) => eyre::bail!(e),
                };

                MetadataWriter::new(&metadata_path).write(&metadata, &n.to_string())?;

                counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                log::info!("Saved {:?} / {} NFTs", counter, self.num);
                Ok(())
            })
            .collect();

        results?;
        Ok(())
    }
}
