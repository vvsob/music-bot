use std::{path::Path, process::Command};

use crate::track::TrackInfo;

#[derive(Debug, Clone, Copy)]
pub struct DownloadError;

pub fn download_from_youtube(url: &str) -> Result<TrackInfo, DownloadError> {
    let output = Command::new("yt-dlp")
        .args([
            "-o",
            "%(id)s",
            "--extract-audio",
            "--audio-format",
            "mp3",
            "--print",
            "%(id)s",
            "--no-simulate",
            url,
        ])
        .output()
        .unwrap();

    if !output.stderr.is_empty() || output.stdout.is_empty() {
        return Err(DownloadError);
    }

    let filename = std::str::from_utf8(output.stdout.as_slice())
        .unwrap()
        .replace('\n', "")
        + ".mp3";

    Ok(TrackInfo::new(Path::new(filename.as_str())))
}
