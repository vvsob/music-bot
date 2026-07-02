use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Weak};

#[derive(Debug)]
pub struct FileCache {
    entries: HashMap<PathBuf, Weak<Entry>>,
}

impl FileCache {
    pub fn new() -> Self {
        Self {entries: HashMap::new()}
    }

    pub fn contains_key(&self, path: &Path) -> bool {
        self.entries.contains_key(path) && self.entries[path].upgrade().is_some()
    }

    pub fn create_file_handle(&mut self, path: &Path) -> FileHandle {
        if let Some(weak) = self.entries.get(path) {
            if let Some(entry) = weak.upgrade() {
                return FileHandle {entry};
            }
        }

        let entry = Arc::new(Entry {path: path.to_path_buf()});
        self.entries.insert(path.to_path_buf(), Arc::downgrade(&entry));
        FileHandle { entry }
    }
}

#[derive(Debug)]
pub struct FileHandle {
    entry: Arc<Entry>
}

impl FileHandle {
    pub fn get_path(&self) -> &Path {
        self.entry.get_path()
    }
}

#[derive(Debug)]
pub struct Entry {
    path: PathBuf,
}

impl Entry {
    pub fn get_path(&self) -> &Path {
        &self.path
    }
}

impl Drop for Entry {
    fn drop(&mut self) {
        let _ = fs::remove_file(self.path.clone());
    }
}
