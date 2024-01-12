use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::{DataProvider, Result};

#[derive(Clone, Debug, Default)]
pub struct FileDataProvider {
    path: PathBuf,
    bytes: Vec<u8>,
    lines: HashMap<i32, String>,
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

    fn get_line(&mut self, line: i32) -> Option<String> {
        if let Some(s) = self.lines.get(&line) {
            return Some(s.to_owned());
        }
        let start = line as usize * 16;
        if start >= self.len() {
            return None;
        }

        let mut end = line as usize * 16 + 16;
        if end > self.len() {
            end = self.len();
        }
        let chunk = self.bytes.get(start..end);
        match chunk {
            Some(bytes) => {
                let mut s = format!("{:08X} ", line * 16);
                for i in 0..bytes.len() {
                    if i < 8 {
                        s.push_str(&format!("{:02X} ", bytes[i]));
                    } else {
                        s.push_str(&format!(" {:02X}", bytes[i]));
                    }
                }
                s.push_str(&format!("{:width$}", "", width = (16 - bytes.len()) * 3));
                s.push_str("  ");
                for b in bytes {
                    let c = match char::from_u32(*b as u32) {
                        Some(c) => {
                            if c.is_ascii_graphic() {
                                c
                            } else {
                                '.'
                            }
                        }
                        None => '.',
                    };
                    s.push(c);
                }
                self.lines.insert(line, s.clone());
                Some(s)
            }
            None => None,
        }
    }
}
