mod gbhiera;
mod ui;

use gbhiera::GbhieraApp;

slint::include_modules!();

fn main() {
    let gbhiera_ui = GbhieraUI::new().unwrap();
    let app = GbhieraApp::new(gbhiera_ui);

    app.run();
}
