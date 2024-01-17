use std::sync::{Arc, RwLock};

use bhiera::{Bhiera, DataProvider, FileDataProvider, Model};
use rfd;
use slint::ComponentHandle;

use crate::GbhieraUI;
use super::hexview::{self, PlotConfig};

pub fn setup(ui: &GbhieraUI, bhiera: Arc<RwLock<Bhiera>>) {
    let plot_config = PlotConfig::new("Courier New", 18.0);
    let handle_weak = ui.as_weak();
    let instance = bhiera.clone();
    let config = plot_config.clone();
    ui.on_show_open_dialog({
        move || {
            let data_provider = load_data_provider(handle_weak.clone());
            if let Some(binary_data) = data_provider {
                apply_data_provider(handle_weak.clone(), &binary_data, &config);
                instance.write().unwrap().set_data_provider(binary_data);
            }
        }
    });
    let instance = bhiera.clone();
    let config = plot_config.clone();
    ui.on_render_plot({
        move |view_start, view_height| {
            let start_line = (view_start + config.char_height as i32 - 1) / config.char_height as i32;
            let line_count = view_height as u32 / config.char_height;
            let view = instance.read().unwrap().get_view(start_line as usize * 16, line_count as usize * 16);
            match view {
                Some(view) => hexview::render_plot(&config, start_line, view_height, view),
                None => slint::Image::default(),
            }
        }
    });
}

fn load_data_provider(handle: slint::Weak<GbhieraUI>) -> Option<FileDataProvider> {
    let mut dialog = rfd::FileDialog::new();
    dialog = dialog.set_title("Select a binary");

    let path = match dialog.pick_file() {
        Some(path) => path,
        None => return None,
    };

    notify(&handle, "Loading data...");
    let binary_data = match FileDataProvider::new(path) {
        Ok(provider) => {
            notify(&handle, "Data loaded");
            provider
        }
        Err(err) => {
            notify(&handle, format!("Loading data...{}", err));
            return None;
        }
    };

    let path_str = binary_data.to_path().to_string_lossy().as_ref().into();
    handle
        .upgrade_in_event_loop(move |h| {
            h.set_binary_path(path_str);
        })
        .unwrap();

    Some(binary_data)
}

fn apply_data_provider(handle: slint::Weak<GbhieraUI>, binary_data: &FileDataProvider, config: &PlotConfig) {
    let hexview_width = config.width;
    let total_line_count = (binary_data.len() + 15) / 16;
    let hexview_height = config.char_height * total_line_count as u32;
    handle
        .upgrade_in_event_loop(move |h| {
            h.set_hexview_width(hexview_width as f32);
            h.set_hexview_height(hexview_height as f32);
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
