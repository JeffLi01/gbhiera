use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::{DataProvider, Result};

#[derive(Clone, Debug, Default)]
pub struct FileDataProvider {
    path: PathBuf,
    bytes: Vec<u8>,
}

impl FileDataProvider {
    pub fn new(path: PathBuf) -> Result<FileDataProvider> {
        let mut bytes = Vec::new();
        let mut f = File::open(&path)?;
        f.read_to_end(&mut bytes)?;
        Ok(Self {
            path,
            bytes,
            ..Default::default()
        })
    }

    pub fn to_path(&self) -> &Path {
        &self.path
    }
}

impl DataProvider for FileDataProvider {
    fn len(&self) -> usize {
        self.bytes.len()
    }

    fn get(&self, offset: usize, count: usize) -> Vec<u8> {
        if offset >= self.len() {
            return Vec::new();
        }

        let mut end = offset + count;
        if end > self.len() {
            end = self.len();
        }
        match self.bytes.get(offset..end) {
            Some(bytes) => bytes.to_owned(),
            None => Vec::new(),
        }
    }
}
