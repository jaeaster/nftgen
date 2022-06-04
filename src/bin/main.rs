use std::fs;

use color_eyre::eyre;

use nftgen::{args, image_builder::ImageBuilder, layer::get_layer_groups};

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let args = args::parse_nftgen_args()?;
    let nft_count = args.num;
    let layers_path = args.layers_path;
    let layers_order = args.layers_order;
    let output_path = args.output_path;

    fs::create_dir_all(output_path.as_path())?;

    let mut layer_groups = get_layer_groups(&layers_path, &layers_order)?;
    layer_groups.sort_by(|a, b| a.partial_cmp(b).unwrap());

    for n in 0..nft_count {
        println!("\nCreating NFT # {}", n);

        let nft = ImageBuilder::build(&layer_groups);

        let outfile_path = output_path.as_path().join(format!("{}.png", n));
        nft.save(&outfile_path)?;

        println!("Saved NFT to {:?}", outfile_path);
    }

    Ok(())
}
