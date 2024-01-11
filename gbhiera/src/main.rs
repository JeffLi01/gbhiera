use std::sync::{Arc, RwLock};

slint::include_modules!();

mod gbhiera;

fn main() {
    let gbhiera_ui = GbhieraUI::new().unwrap();
    let app = Arc::new(RwLock::new(gbhiera::GbhieraApp::new()));

    gbhiera::setup(&gbhiera_ui, app.clone());

    gbhiera_ui.run().expect("failed to run");
}
