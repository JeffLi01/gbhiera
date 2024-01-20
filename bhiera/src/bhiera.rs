use crate::DataProvider;

#[derive(Clone, Copy, Default)]
pub struct BhieraGeometry {
    pub char_width: u32,
    pub char_height: u32,
    pub hex_byte_width: u32,
    pub offset_view_width: u32,
}

impl BhieraGeometry {
    pub fn width(&self) -> u32 {
        let right_margin = self.char_width;
        self.offset_view_width + self.hex_view_width() + self.char_view_width() + right_margin
    }

    pub fn hex_view_width(&self) -> u32 {
        (self.char_width + self.hex_byte_width) * 16 + self.char_width * 2
    }

    pub fn char_view_width(&self) -> u32 {
        self.char_width * 16
    }
}

#[derive(Default)]
pub struct Bhiera {
    data_provider: Option<Box<dyn DataProvider>>,
    plot_config: BhieraGeometry,
}

impl Bhiera {
    pub fn new() -> Self {
        Self {
            data_provider: None,
            ..Default::default()
        }
    }

    pub fn set_geometry(&mut self, config: &BhieraGeometry) {
        self.plot_config = *config;
    }
}

#[derive(Default)]
pub struct View<'a> {
    offset: usize,
    bytes: &'a [u8],
    current: usize,
}

impl View<'_> {
    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn size(&self) -> usize {
        self.bytes.len()
    }
}

impl Iterator for View<'_> {
    type Item = Element;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.bytes.len() {
            return None;
        }
        let element = Element::Byte(TextElement { byte: self.bytes[self.current] });
        let result = Some(element);
        self.current += 1;
        result
    }
}

pub struct TextElement {
    pub byte: u8,
}

pub enum Element {
    Byte(TextElement),
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
            return bytes.map(|bytes|
                View {
                    offset,
                    bytes,
                    ..Default::default()
                });
        }
        None
    }
}
