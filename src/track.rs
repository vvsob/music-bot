use std::path::{Path, PathBuf};
#[derive(Debug, Clone)]
pub struct TrackInfo {
    pub path: PathBuf,
}

impl TrackInfo {
    pub fn new(path: &Path) -> TrackInfo {
        TrackInfo {
            path: PathBuf::from(path),
        }
    }
}
