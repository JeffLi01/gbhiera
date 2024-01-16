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
    fn get_bytes(&self, offset: usize, count: usize) -> Vec<u8>;
}

impl View for Bhiera {
    fn set_data_provider(&mut self, provider: impl DataProvider + 'static) {
        self.data_provider.replace(Box::new(provider));
    }
    fn get_bytes(&self, offset: usize, count: usize) -> Vec<u8> {
        if let Some(binary_data) = &self.data_provider {
            let bytes = (*binary_data).get(offset, count);
            return bytes;
        }
        vec![]
    }
}
