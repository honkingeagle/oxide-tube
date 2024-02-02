use color_eyre::Result;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use rustube::{Callback, Stream};
use std::fmt::Write;

pub fn progress(stream: Stream) -> Result<()> {
    let total_size = stream.blocking_content_length()?;
    let pb = ProgressBar::new(total_size);
    let new_pb = pb.clone();

    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
    .unwrap()
    .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
    .progress_chars("#>-"));

    let callback = Callback::new();

    let callback = callback.connect_on_progress_closure_slow(move |args| {
        pb.set_position(args.current_chunk as u64);
    });

    let callback = callback.connect_on_complete_closure(move |_| {
        new_pb.finish_with_message("Your video is ready 🎊");
    });

    stream.blocking_download_with_callback(callback)?;

    Ok(())
}