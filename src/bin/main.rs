use color_eyre::eyre;
use pretty_env_logger;

use nftgen::cmd::{
    opts::{Opts, Subcommands},
    Cmd,
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    pretty_env_logger::init();

    let opts = Opts::parse_from_config_and_cli()?;
    match opts.sub {
        Subcommands::Generate(cmd) => cmd.run()?,
        Subcommands::Upload(cmd) => cmd.run()?.await?,
    }

    Ok(())
}
