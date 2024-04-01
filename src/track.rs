use std::path::{Path, PathBuf};
#[derive(Debug, Clone)]
pub struct Track {
    pub path: PathBuf,
}

impl Track {
    pub fn new(path: &Path) -> Track {
        Track {
            path: PathBuf::from(path),
        }
    }
}
