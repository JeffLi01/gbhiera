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

pub struct View<'a> {
    pub bytes: &'a [u8],
}

pub trait Model {
    fn set_data_provider(&mut self, provider: impl DataProvider + 'static);
    fn get_view(&self, offset: usize, count: usize) -> Option<View>;
}

impl Model for Bhiera {
    fn set_data_provider(&mut self, provider: impl DataProvider + 'static) {
        self.data_provider.replace(Box::new(provider));
    }
    fn get_view(&self, offset: usize, count: usize) -> Option<View> {
        if let Some(binary_data) = &self.data_provider {
            let bytes = (*binary_data).get(offset, count);
            return bytes.map(|bytes| View {bytes});
        }
        None
    }
}
