use crate::file::FileHandle;

#[derive(Debug, Clone)]
pub struct TrackInfo {
    pub name: String,
}

impl TrackInfo {
    pub fn new(name: &str) -> TrackInfo {
        TrackInfo { name: String::from(name) }
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
