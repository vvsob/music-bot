use crate::file::FileHandle;

#[derive(Debug, Clone)]
pub struct Track {
    pub file: FileHandle,
}

impl Track {
    pub fn new(file: FileHandle) -> Track {
        Track { file }
    }
}
