use crate::file::FileHandle;

#[derive(Debug, Clone)]
pub struct TrackInfo {
    pub url: String,
    pub name: String,
}

impl TrackInfo {
    pub fn new(url: &str, name: &str) -> TrackInfo {
        TrackInfo { url: url.to_string(), name: name.to_string() }
    }
}

#[derive(Debug)]
pub struct Track {
    pub info: TrackInfo,
    pub file: FileHandle,
}

impl Track {
    pub fn new(info: TrackInfo, file: FileHandle) -> Track {
        Track { info, file }
    }
}
