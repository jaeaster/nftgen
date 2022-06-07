use std::{fs, sync::atomic::AtomicU32};

use color_eyre::eyre;
use log;
use pretty_env_logger;
use rayon::prelude::*;

use nftgen::{
    args, image_builder::ImageBuilder, layer::get_layer_groups, metadata::MetadataBuilder,
};

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    pretty_env_logger::init();

    let args = args::parse_nftgen_args()?;
    log::info!("Starting nftgen with args: {:?}", args);

    let nft_count = args.num;
    let layers_path = args.layers_path;
    let layers_order = args.layers_order;
    let output_path = args.output_path;
    let images_path = output_path.as_path().join("images");
    let metadata_path = output_path.as_path().join("metadata");

    fs::create_dir_all(images_path.as_path())?;
    fs::create_dir_all(metadata_path.as_path())?;

    log::debug!("Parsing layer groups");
    let mut layer_groups = get_layer_groups(&layers_path, &layers_order)?;
    log::debug!(
        "Sorting layer groups according to order: {}",
        layers_order.join(", ")
    );
    layer_groups.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let counter = AtomicU32::new(0);
    log::debug!("Creating Images and Metadata");
    let results: eyre::Result<Vec<()>> = (0..nft_count)
        .into_par_iter()
        .map(|n| {
            let image_file_path = images_path.as_path().join(format!("{}.png", n));
            let metadata_file_path = metadata_path.as_path().join(format!("{}", n));

            let (nft, layers) = ImageBuilder::build(&layer_groups);
            let metadata = MetadataBuilder::build(
                n as u32,
                &args.description,
                &args.collection_name,
                &args.base_uri,
                &layers_order,
                &layers,
            );

            let metadata_json = serde_json::to_string(&metadata)?;

            log::debug!(
                "Writing image to file: {}",
                image_file_path.to_string_lossy()
            );
            match nft.save_with_format(&image_file_path, image::ImageFormat::Png) {
                Ok(_) => log::debug!("Saved image to file: {}", image_file_path.to_string_lossy()),
                Err(e) => eyre::bail!(e),
            };

            log::debug!(
                "Writing metadata to file: {}",
                metadata_file_path.to_string_lossy()
            );
            fs::write(&metadata_file_path, metadata_json)?;
            log::debug!(
                "Saved metadata to file: {}",
                metadata_file_path.to_string_lossy()
            );

            counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            log::info!("Saved {:?} / {} NFTs", counter, nft_count);
            Ok(())
        })
        .collect();
    results?;

    Ok(())
}
