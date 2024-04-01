use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct FileHandle {
    path: PathBuf,
}

impl FileHandle {
    pub fn new(path: &Path) -> FileHandle {
        FileHandle {
            path: PathBuf::from(path),
        }
    }

    pub fn get_path(&self) -> &Path {
        &self.path
    }
}

impl Drop for FileHandle {
    fn drop(&mut self) {
        let _ = fs::remove_file(self.path.clone());
    }
}
