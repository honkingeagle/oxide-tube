use color_eyre::Result;
use rustube::video_info::player_response::streaming_data::Quality;
use rustube::{blocking::Video, Id, Stream};

pub fn pick_stream(url: String) -> Result<Stream> {
    let id = Id::from_raw(&url)?;
    let streams = Video::from_id(id.into_owned())?.into_streams();

    let stream: Vec<_> = streams
        .into_iter()
        .filter(|s| s.quality == Quality::Hd720)
        .collect();

    let stream = stream.get(0).unwrap().to_owned();

    Ok(stream)
}
