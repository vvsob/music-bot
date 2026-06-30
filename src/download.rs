use std::{path::Path};

use serde_json;
use tokio::process::Command;

use crate::{file::FileHandle, track::{Track, TrackInfo}};

#[derive(Debug, Clone, Copy)]
pub struct DownloadError;

pub async fn download_from_youtube(url: &str) -> Result<Track, DownloadError> {
    let output = Command::new("yt-dlp")
        .args([
            "--print",
            "%(id)s %(duration)i",
            "--no-playlist",
            "--no-warnings",
            "--",
            url,
        ]).output().await.unwrap();

    let items: Vec<String> = std::str::from_utf8(&output.stdout).unwrap().split(' ').map(|s| s.replace("\n", "")).collect();

    if items.len() < 2 {
        return Err(DownloadError);        
    }

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
            "--write-info-json",
            "--",
            url,
        ])
        .output().await.unwrap();

    let info_filename = items[0].clone() + ".info.json";
    let info_json = std::fs::read_to_string(&info_filename).unwrap();

    let info: serde_json::Value = serde_json::from_str(&info_json).unwrap();
    let title = info["title"].as_str().unwrap();

    println!("{}", title);

    std::fs::remove_file(info_filename).unwrap();

    if !output.stderr.is_empty() || output.stdout.is_empty() {
        println!("{}", std::str::from_utf8(output.stderr.as_slice()).unwrap());
        println!("{}", std::str::from_utf8(output.stdout.as_slice()).unwrap());
        return Err(DownloadError);
    }

    let file_handle = FileHandle::new(Path::new(filename.as_str()));
    let info = TrackInfo::new(&format!("https://youtu.be/{}", items[0]), title);

    Ok(Track::new(info, file_handle))
}
