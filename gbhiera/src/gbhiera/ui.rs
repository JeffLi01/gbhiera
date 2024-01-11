use std::sync::{Arc, RwLock};

use bhiera::{Bhiera, BhieraImpl, DataProvider, FileDataProvider};
use rfd;
use slint::ComponentHandle;
use tokio::{self, runtime::Runtime};

use crate::GbhieraUI;

pub fn setup(ui: &GbhieraUI, bhiera: Arc<RwLock<BhieraImpl>>) {
    let handle_weak = ui.as_weak();
    let instance = bhiera.clone();
    ui.on_show_open_dialog({
        move || {
            let binary_data = show_open_dialog(handle_weak.clone());
            if let Some(binary_data) = binary_data {
                apply_binary_data(handle_weak.clone(), &binary_data);
                instance.write().unwrap().set_data_provider(binary_data);
            }
        }
    });
    let instance = bhiera.clone();
    ui.on_get_line({
        move |line| {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                match instance.write().unwrap().get_line(line) {
                    Some(s) => s.into(),
                    None => "".into(),
                }
            })
        }
    });
}

fn show_open_dialog(handle: slint::Weak<GbhieraUI>) -> Option<FileDataProvider> {
    let mut dialog = rfd::FileDialog::new();
    dialog = dialog.set_title("Select a binary");

    let binary_data = dialog.pick_file().map(|p| FileDataProvider::from(p));

    if binary_data.is_none() {
        return None;
    }

    let mut binary_data = binary_data.unwrap();
    let path_str = binary_data.to_path().to_string_lossy().as_ref().into();
    handle
        .upgrade_in_event_loop(move |h| {
            h.set_binary_path(path_str);
        })
        .unwrap();
    notify(&handle, "Loading binary...");

    match binary_data.load() {
        Ok(_) => notify(&handle, "Binary loaded"),
        Err(e) => notify(&handle, format!("{}", e)),
    }
    Some(binary_data)
}

fn apply_binary_data(handle: slint::Weak<GbhieraUI>, binary_data: &FileDataProvider) {
    let total_line_count = ((binary_data.len() + 15) / 16) as i32;
    handle
        .upgrade_in_event_loop(move |h| {
            h.set_total_line_count(total_line_count);
        })
        .unwrap();
}

fn notify<S>(handle: &slint::Weak<GbhieraUI>, msg: S)
where
    S: Into<String>,
{
    let msg = msg.into();
    handle
        .upgrade_in_event_loop(move |h| {
            h.set_status(msg.into());
        })
        .unwrap();
}
