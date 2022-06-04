use color_eyre::eyre;

use nftgen::{args, layer::get_layer_groups};

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let args = args::parse_nftgen_args()?;
    let nft_count = args.num;
    let layers_path = args.layers_path;
    let layers_order = args.layers_order;

    let mut layer_groups = get_layer_groups(&layers_path, &layers_order)?;
    layer_groups.sort_by(|a, b| a.partial_cmp(b).unwrap());

    for n in 0..nft_count {
        println!("\nNFT # {}", n);
        for layer_group in layer_groups.iter() {
            let layer = layer_group.pick();
            println!("{}-{}", layer_group.layer_type, layer.name);
        }
    }

    Ok(())
}
