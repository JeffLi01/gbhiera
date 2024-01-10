use super::GbhieraUI;
use bhiera::FileLoader;
use futures::future::FutureExt;
use rfd;
use slint::ComponentHandle;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot::Sender;

#[derive(Debug)]
pub enum GbhieraMessage {
    Quit,
    ShowOpenDialog,
    Expose { line: i32, sender: Sender<String> },
}

pub struct GbhieraApp {
    pub channel: UnboundedSender<GbhieraMessage>,
    worker_thread: std::thread::JoinHandle<()>,
}

impl GbhieraApp {
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
    let mut binary_data = None;
    loop {
        let m = futures::select! {
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
                binary_data = show_open_dialog(handle.clone());
                if let Some(binary_data) = &binary_data {
                    apply_binary_data(&binary_data, handle.clone());
                }
            }
            GbhieraMessage::Expose { line, sender } => {
                if let Some(ref mut binary_data) = &mut binary_data {
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

fn show_open_dialog(handle: slint::Weak<GbhieraUI>) -> Option<FileLoader> {
    let mut dialog = rfd::FileDialog::new();
    dialog = dialog.set_title("Select a binary");

    let binary_data = dialog.pick_file().map(|p| FileLoader::from(p));

    if binary_data.is_none() {
        return None;
    }

    let mut binary_data = binary_data.unwrap();
    let path_str = binary_data.to_path().to_string_lossy().as_ref().into();
    handle
        .clone()
        .upgrade_in_event_loop(move |h| {
            h.set_binary_path(path_str);
            h.set_status("Loading binary...".into());
        })
        .unwrap();

    match binary_data.load() {
        Ok(_) => {
            handle
                .clone()
                .upgrade_in_event_loop(move |h| {
                    h.set_status("Binary loaded".into());
                })
                .unwrap();
        }
        Err(e) => {
            handle
                .upgrade_in_event_loop(move |h| {
                    h.set_status(format!("{}", e).into());
                })
                .unwrap();
        }
    }
    Some(binary_data)
}

fn apply_binary_data(binary_data: &FileLoader, handle: slint::Weak<GbhieraUI>) {
    let total_line_count = ((binary_data.len() + 15) / 16) as i32;
    handle
        .clone()
        .upgrade_in_event_loop(move |h| {
            h.set_total_line_count(total_line_count);
        })
        .unwrap();
}
