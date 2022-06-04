use std::{fs, sync::atomic::AtomicU32};

use color_eyre::eyre;
use rayon::prelude::*;

use nftgen::{
    args, image_builder::ImageBuilder, layer::get_layer_groups, metadata::MetadataBuilder,
};

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let args = args::parse_nftgen_args()?;
    let nft_count = args.num;
    let layers_path = args.layers_path;
    let layers_order = args.layers_order;
    let output_path = args.output_path;
    let images_path = output_path.as_path().join("images");
    let metadata_path = output_path.as_path().join("metadata");

    fs::create_dir_all(images_path.as_path())?;
    fs::create_dir_all(metadata_path.as_path())?;

    let mut layer_groups = get_layer_groups(&layers_path, &layers_order)?;
    layer_groups.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let counter = AtomicU32::new(0);
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

            match nft.save(&image_file_path) {
                Ok(_) => (),
                Err(e) => eyre::bail!(e),
            };
            fs::write(&metadata_file_path, metadata_json)?;

            counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            println!("Saved {:?} / {} NFTs", counter, nft_count);
            Ok(())
        })
        .collect();
    results?;

    Ok(())
}
