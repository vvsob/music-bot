use std::{path::Path, process::Command};

use crate::{file::FileHandle, track::{Track, TrackInfo}};

#[derive(Debug, Clone, Copy)]
pub struct DownloadError;

pub fn download_from_youtube(url: &str) -> Result<Track, DownloadError> {
    let output = Command::new("yt-dlp")
        .args([
            "--print",
            "%(id)s %(duration)i",
            "--no-playlist",
            "--no-warnings",
            "--",
            url,
        ]).output().unwrap();

    let items: Vec<String> = std::str::from_utf8(&output.stdout).unwrap().split(' ').map(|s| s.replace("\n", "")).collect();

    let filename = items[0].clone() + ".mp3";
    let duration: u32 = items[1].parse().unwrap();

    println!("{}", filename);
    println!("{}", duration);

    if duration > 900 {
        return Err(DownloadError);
    }

    let output = Command::new("yt-dlp")
        .args([
            "-o",
            "%(id)s",
            "--extract-audio",
            "--audio-format",
            "mp3",
            "--no-playlist",
            "--no-warnings",
            "--",
            url,
        ])
        .output()
        .unwrap();

    if !output.stderr.is_empty() || output.stdout.is_empty() {
        println!("{}", std::str::from_utf8(output.stderr.as_slice()).unwrap());
        println!("{}", std::str::from_utf8(output.stdout.as_slice()).unwrap());
        return Err(DownloadError);
    }

    let file_handle = FileHandle::new(Path::new(filename.as_str()));
    let info = TrackInfo::new(&filename);

    Ok(Track::new(info, file_handle))
}
