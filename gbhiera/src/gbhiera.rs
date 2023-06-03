use std::collections::HashMap;
use std::path::{PathBuf, Path};
use std::fs::File;
use std::io::Read;


use rfd;
use tokio::sync::oneshot::Sender;
use super::GbhieraUI;
use futures::future::FutureExt;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use slint::ComponentHandle;

#[derive(Debug)]
pub enum GbhieraMessage {
    Quit,
    ShowOpenDialog,
    Expose {
        line: i32,
        sender: Sender<String>,
    },
}

pub struct GbhieraWorker {
    pub channel: UnboundedSender<GbhieraMessage>,
    worker_thread: std::thread::JoinHandle<()>,
}

impl GbhieraWorker {
    pub fn new(gbhiera_ui: &GbhieraUI) -> Self {
        let (channel, r) = tokio::sync::mpsc::unbounded_channel();
        let worker_thread = std::thread::spawn({
            let handle_weak = gbhiera_ui.as_weak();
            move || {
                tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(gbhiera_worker_loop(r, handle_weak))
                    .unwrap()
            }
        });
        Self {
            channel,
            worker_thread,
        }
    }

    pub fn join(self) -> std::thread::Result<()> {
        let _ = self.channel.send(GbhieraMessage::Quit);
        self.worker_thread.join()
    }
}


async fn gbhiera_worker_loop(
    mut r: UnboundedReceiver<GbhieraMessage>,
    handle: slint::Weak<GbhieraUI>,
) -> tokio::io::Result<()> {
    let mut binary: Option<Binary> = None;
    let mut binary_data: Option<BinaryData> = None;

    let read_binary_future = read_binary(binary.clone(), handle.clone()).fuse();
    futures::pin_mut!(
        read_binary_future,
    );
    loop {
        let m = futures::select! {
            res = read_binary_future => {
                binary_data = res;
                if let Some(binary_data) = &binary_data {
                    apply_binary_data(binary_data, handle.clone());
                }
                continue;
            }
            m = r.recv().fuse() => {
                match m {
                    None => return Ok(()),
                    Some(m) => m,
                }
            }
        };

        match m {
            GbhieraMessage::Quit => return Ok(()),
            GbhieraMessage::ShowOpenDialog => {
                binary = show_open_dialog();
                read_binary_future.set(read_binary(binary.clone(), handle.clone()).fuse());
            }
            GbhieraMessage::Expose{line, sender} => {
                if let Some(binary_data) = &mut binary_data {
                    if let Some(s) = binary_data.get_line(line) {
                        let _ = sender.send(s);
                        continue;
                    }
                }
                let _ = sender.send("".into());
            }
        }
    }
}

fn show_open_dialog() -> Option<Binary> {
    let mut dialog = rfd::FileDialog::new();
    dialog = dialog.set_title("Select a binary");

    dialog.pick_file().map(|p| Binary::from(p))
}

#[derive(Debug, Clone)]
struct Binary(PathBuf);

impl From<PathBuf> for Binary {
    fn from(file: PathBuf) -> Self {
        Self(file)
    }
}

impl Binary {
    fn to_path(&self) -> &Path {
        &self.0
    }
}

#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
struct BinaryData {
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

async fn read_binary(binary: Option<Binary>, handle: slint::Weak<GbhieraUI>) -> Option<BinaryData> {
    if binary.is_none() {
        return None;
    }
    let binary_str = binary
        .clone()
        .unwrap()
        .to_path()
        .to_string_lossy()
        .as_ref()
        .into();
    handle
        .clone()
        .upgrade_in_event_loop(move |h| {
            h.set_binary_path(binary_str);
            h.set_status("Loading binary...".into());
        })
        .unwrap();
    match BinaryData::load(binary.unwrap()) {
        Ok(binary_data) => {
            handle
                .clone()
                .upgrade_in_event_loop(move |h| {
                    h.set_status("Binary loaded".into());
                })
                .unwrap();

            Some(binary_data)
        }
        Err(e) => {
            handle
                .upgrade_in_event_loop(move |h| {
                    h.set_status(format!("{}", e).into());
                })
                .unwrap();
            None
        }
    }
}

fn apply_binary_data(binary_data: &BinaryData, handle: slint::Weak<GbhieraUI>) {
    let total_line_count = ((binary_data.len() + 15) / 16) as i32;
    handle
        .clone()
        .upgrade_in_event_loop(move |h| {
            h.set_total_line_count(total_line_count);
        })
        .unwrap();
}
