use crate::DataProvider;

pub struct BhieraImpl {
    data_provider: Option<Box<dyn DataProvider>>,
}

impl BhieraImpl {
    pub fn new() -> Self {
        Self {
            data_provider: None,
        }
    }
}

pub trait Bhiera {
    fn set_data_provider(&mut self, provider: impl DataProvider + 'static);
    fn get_line(&mut self, line: i32) -> Option<String>;
}

impl Bhiera for BhieraImpl {
    fn set_data_provider(&mut self, provider: impl DataProvider + 'static) {
        self.data_provider.replace(Box::new(provider));
    }
    fn get_line(&mut self, line: i32) -> Option<String> {
        if let Some(binary_data) = &mut self.data_provider {
            if let Some(s) = (*binary_data).get_line(line) {
                return Some(s.into());
            }
        }
        None
    }
}
