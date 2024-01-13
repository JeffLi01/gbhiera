use std::sync::{Arc, RwLock};

use bhiera::Bhiera;
use slint::ComponentHandle;

use crate::GbhieraUI;

pub struct GbhieraApp {
    bhiera: Arc<RwLock<Bhiera>>,
    ui: GbhieraUI,
}

impl GbhieraApp {
    pub fn new(ui: GbhieraUI) -> Self {
        Self {
            bhiera: Arc::new(RwLock::new(Bhiera::new())),
            ui,
        }
    }

    pub fn run(&self) {
        crate::ui::setup(&self.ui, self.bhiera.clone());
        self.ui.run().unwrap();
    }
}
