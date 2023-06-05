use std::collections::HashMap;
use std::path::{PathBuf, Path};
use std::fs::File;
use std::io::Read;

use anyhow;

#[derive(Debug, Clone)]
pub struct Binary(PathBuf);

impl From<PathBuf> for Binary {
    fn from(file: PathBuf) -> Self {
        Self(file)
    }
}

impl Binary {
    pub fn to_path(&self) -> &Path {
        &self.0
    }
}

#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub struct BinaryData {
    bytes: Vec<u8>,
    lines: HashMap<i32, String>,
}

impl BinaryData {
    pub fn load(binary: Binary) -> anyhow::Result<Self> {
        let mut f = File::open(binary.to_path())?;
        let mut bytes = vec![];
        f.read_to_end(&mut bytes)?;
        Ok(BinaryData{
            bytes,
            ..Default::default()
        })
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn get_line(&mut self, line: i32) -> Option<String> {
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
                            if c.is_ascii_graphic() { c } else { '.' }
                        }
                        None => '.'
                    };
                    s.push(c);
                }
                self.lines.insert(line, s.clone());
                Some(s)
            }
            None => {
                None
            }
        }
    }
}
