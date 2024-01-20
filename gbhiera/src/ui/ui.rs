use std::sync::{Arc, RwLock};

use bhiera::{Bhiera, FileDataProvider, Model};
use rfd;
use slint::ComponentHandle;

use super::Plotter;
use crate::GbhieraUI;

pub fn setup(ui: &GbhieraUI, bhiera: Arc<RwLock<Bhiera>>) {
    let orig_plotter = Plotter::with_font("Courier New", 18.0);
    bhiera.write().unwrap().set_geometry(&orig_plotter.config);
    let handle_weak = ui.as_weak();
    let instance = bhiera.clone();
    let plotter = orig_plotter.clone();
    ui.on_show_open_dialog({
        move || {
            let data_provider = load_data_provider(handle_weak.clone());
            if let Some(binary_data) = data_provider {
                let (hexview_width, hexview_height) = plotter.geometry(&binary_data);
                handle_weak
                    .upgrade_in_event_loop(move |h| {
                        h.set_hexview_width(hexview_width as f32);
                        h.set_hexview_height(hexview_height as f32);
                    })
                    .unwrap();

                instance.write().unwrap().set_data_provider(binary_data);
            }
        }
    });
    let instance = bhiera.clone();
    let plotter = orig_plotter.clone();
    ui.on_render_plot({
        move |view_start, view_height| {
            let bhiera = instance.read().unwrap();
            plotter.plot(&bhiera, view_start, view_height)
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

    update_status(&handle, "Loading data...");
    let binary_data = match FileDataProvider::new(path) {
        Ok(provider) => {
            update_status(&handle, "Data loaded");
            provider
        }
        Err(err) => {
            update_status(&handle, format!("Loading data...{}", err));
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

fn update_status<S>(handle: &slint::Weak<GbhieraUI>, msg: S)
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
