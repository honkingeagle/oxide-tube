use clap::Parser;
use color_eyre::Result;
use oxide_tube::Args;

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    oxide_tube::download(args.url)?;

    Ok(())
}
