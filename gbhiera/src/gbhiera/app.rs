use std::sync::{Arc, RwLock};

use bhiera::BhieraImpl;
use slint::ComponentHandle;

use crate::GbhieraUI;

pub struct GbhieraApp {
    bhiera: Arc<RwLock<BhieraImpl>>,
    ui: GbhieraUI,
}

impl GbhieraApp {
    pub fn new(ui: GbhieraUI) -> Self {
        Self {
            bhiera: Arc::new(RwLock::new(BhieraImpl::new())),
            ui,
        }
    }

    pub fn run(&self) {
        super::setup(&self.ui, self.bhiera.clone());
        self.ui.run().unwrap();
    }
}
