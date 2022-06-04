use std::fs;

use color_eyre::eyre;

use nftgen::{
    args,
    image_builder::ImageBuilder,
    layer::get_layer_groups,
    metadata::{Attribute, Metadata},
};
use rayon::prelude::*;

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

    let results: eyre::Result<Vec<()>> = (0..nft_count)
        .into_par_iter()
        .map(|n| {
            let image_file_path = images_path.as_path().join(format!("{}.png", n));
            let metadata_file_path = metadata_path.as_path().join(format!("{}", n));

            let (nft, layers) = ImageBuilder::build(&layer_groups);

            let attributes: Vec<Attribute> = layers_order
                .iter()
                .zip(layers.iter().map(|&l| l.name.as_str()))
                .map(|(layer_type, layer_name)| {
                    Attribute::new(layer_type.to_string(), layer_name.to_string())
                })
                .collect();

            let metadata = Metadata::new(
                &args.description,
                format!("{} #{}", args.collection_name, n),
                format!("{}/{}.png", args.base_uri, n),
                attributes,
            );
            let metadata_json = serde_json::to_string(&metadata)?;

            match nft.save(&image_file_path) {
                Ok(_) => (),
                Err(e) => eyre::bail!(e),
            };
            fs::write(&metadata_file_path, metadata_json)?;

            println!("Saved NFT to {:?}", image_file_path);
            Ok(())
        })
        .collect();
    results?;

    Ok(())
}
