use std::{collections::HashMap, path::Path};

use serde_json;
use tokio::process::Command;

use crate::{file::FileCache, track::{Track, TrackInfo}};

pub struct Downloader {
    cache: FileCache,
    track_infos: HashMap<String, TrackInfo>,
}

impl Downloader {
    pub fn new() -> Self {
        Self {cache: FileCache::new(), track_infos: HashMap::new()}
    }

    pub async fn download_from_youtube(&mut self, url: &str) -> Result<Track, DownloadError> {
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
        
        let id = items[0].clone();

        let filename = id.clone() + ".mp3";
        let duration: u32 = items[1].parse().unwrap();

        println!("{}", filename);
        println!("{}", duration);

        if duration > 900 {
            return Err(DownloadError);
        }

        let file_path = Path::new(filename.as_str());

        if self.cache.contains_key(file_path) {
            let file_handle = self.cache.create_file_handle(file_path);
            return Ok(Track { info: self.track_infos[&id].clone(), file: file_handle });
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

        let info_filename = id.clone() + ".info.json";
        let info_json = std::fs::read_to_string(&info_filename).unwrap();

        let info: serde_json::Value = serde_json::from_str(&info_json).unwrap();
        let title = info["title"].as_str().unwrap();

        println!("{}", title);

        std::fs::remove_file(info_filename).unwrap();

        if !output.stderr.is_empty() || output.stdout.is_empty() {
            println!("===== Stderr\n{}", std::str::from_utf8(output.stderr.as_slice()).unwrap());
            println!("===== Stdout\n{}", std::str::from_utf8(output.stdout.as_slice()).unwrap());
            return Err(DownloadError);
        }

        let _ = Command::new("uvx")
            .args([
                "ffmpeg-normalize",
                &filename,
                "-ofmt",
                "mp3",
                "-o",
                &filename,
                "-f"
            ]).output().await.unwrap();

        let info = TrackInfo::new(&format!("https://youtu.be/{}", id), title);
        self.track_infos.insert(id, info.clone());

        let file_handle = self.cache.create_file_handle(file_path);

        Ok(Track { info, file: file_handle })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DownloadError;
