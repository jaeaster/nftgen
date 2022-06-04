use color_eyre::eyre;

use nftgen::{args, layer::get_layer_groups};

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let args = args::parse_nftgen_args()?;
    let nft_count = args.num;
    let layers_path = args.layers_path;

    let layer_groups = get_layer_groups(&layers_path)?;

    for n in 0..nft_count {
        println!("\nNFT # {}", n);
        for layer_group in layer_groups.iter() {
            let layer = layer_group.pick();
            println!("{}", layer.name);
        }
    }

    Ok(())
}
