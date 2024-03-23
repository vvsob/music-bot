use crate::track::TrackInfo;
use std::{path::Path, process::Command};

pub fn download_from_youtube(url: &str) -> TrackInfo {
    let output = Command::new("yt-dlp")
        .args([
            "-o",
            "%(id)s",
            "--extract-audio",
            "--audio-format",
            "mp3",
            "--print",
            "%(id)s",
            url,
        ])
        .output()
        .unwrap();

    let filename = std::str::from_utf8(output.stdout.as_slice())
        .unwrap()
        .replace('\n', "")
        + ".mp3";

    TrackInfo::new(&Path::new(filename.as_str()))
}
