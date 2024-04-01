use std::{path::Path, process::Command};

use crate::{file::FileHandle, track::Track};

#[derive(Debug, Clone, Copy)]
pub struct DownloadError;

pub fn download_from_youtube(url: &str) -> Result<Track, DownloadError> {
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
            "--no-playlist",
            "--",
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

    let file_handle = FileHandle::new(Path::new(filename.as_str()));

    Ok(Track::new(file_handle))
}
