mod progress;
mod stream;

use clap::Parser;
use color_eyre::Result;

/// Yet another youtube video downloader
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Url of the video
    #[arg(short, long)]
    pub url: String,
}

pub fn download(url: String) -> Result<()> {
    let stream = stream::pick_stream(url)?;

    progress::progress(stream)?;

    Ok(())
}
