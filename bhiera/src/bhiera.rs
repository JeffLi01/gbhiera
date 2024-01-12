use crate::DataProvider;

pub struct Bhiera {
    data_provider: Option<Box<dyn DataProvider>>,
}

impl Bhiera {
    pub fn new() -> Self {
        Self {
            data_provider: None,
        }
    }
}

pub trait View {
    fn set_data_provider(&mut self, provider: impl DataProvider + 'static);
    fn get_line(&mut self, line: i32) -> Option<String>;
}

impl View for Bhiera {
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
