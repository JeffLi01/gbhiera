use crate::FileDataProvider;

pub struct BhieraImpl {
    data_provider: Option<FileDataProvider>,
}

impl BhieraImpl {
    pub fn new() -> Self {
        Self {
            data_provider: None,
        }
    }
}

pub trait Bhiera {
    fn set_data_provider(&mut self, provider: FileDataProvider);
    fn get_line(&mut self, line: i32) -> Option<String>;
}

impl Bhiera for BhieraImpl {
    fn set_data_provider(&mut self, provider: FileDataProvider) {
        self.data_provider.replace(provider);
    }
    fn get_line(&mut self, line: i32) -> Option<String> {
        if let Some(ref mut binary_data) = &mut self.data_provider {
            if let Some(s) = binary_data.get_line(line) {
                return Some(s.into());
            }
        }
        None
    }
}
