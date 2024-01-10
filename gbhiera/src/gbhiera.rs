use std::sync::{RwLock, Arc};

use super::GbhieraUI;
use bhiera::{FileLoader, DataProvider};
use rfd;
use slint::ComponentHandle;

use tokio::{self, runtime::Runtime};

pub struct GbhieraApp {
    data_provider: Option<FileLoader>,
}

impl GbhieraApp {
    pub fn new() -> Self {
        Self {
            data_provider: None,
        }
    }
}

pub fn setup(ui: &GbhieraUI, app: Arc<RwLock<GbhieraApp>>) {
    let handle_weak = ui.as_weak();
    let instance = app.clone();
    ui.on_show_open_dialog({
        move || {
            let binary_data = show_open_dialog(handle_weak.clone());
            if let Some(binary_data) = binary_data {
                apply_binary_data(&binary_data, handle_weak.clone());
                instance.write().unwrap().data_provider.replace(binary_data);
            }
        }
    });
    let instance = app.clone();
    ui.on_get_line({
        move |line| {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                if let Some(ref mut binary_data) = &mut instance.write().unwrap().data_provider {
                    if let Some(s) = binary_data.get_line(line) {
                        return s.into();
                    }
                }
                "".into()
            })
        }
    });
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
