use gbhiera::GbhieraApp;

slint::include_modules!();

mod gbhiera;

fn main() {
    let gbhiera_ui = GbhieraUI::new().unwrap();
    let app = GbhieraApp::new(gbhiera_ui);

    app.run();
}
