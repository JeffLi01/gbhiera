use std::collections::VecDeque;

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
pub struct View {
    offset: usize,
    elements: VecDeque<Element>,
}

impl View {
    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }
}

impl Iterator for View {
    type Item = Element;
    fn next(&mut self) -> Option<Self::Item> {
        self.elements.pop_front()
    }
}

pub struct TextElement {
    pub text: String,
    pub x: i32,
    pub y: i32,
    pub fg: (u8, u8, u8),
}

pub struct RectangleElement {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub bg: (u8, u8, u8),
}

pub enum Element {
    Byte(TextElement),
    Rectangle(RectangleElement),
}

pub trait Model {
    fn set_data_provider(&mut self, provider: impl DataProvider + 'static);
    fn get_view(&self, view_start: u32, view_height: u32) -> Option<View>;
}

impl Model for Bhiera {
    fn set_data_provider(&mut self, provider: impl DataProvider + 'static) {
        self.data_provider.replace(Box::new(provider));
    }

    fn get_view(&self, view_start: u32, view_height: u32) -> Option<View> {
        if let Some(binary_data) = &self.data_provider {
            let start_line = (view_start + self.plot_config.char_height - 1) / self.plot_config.char_height;
            let line_count = view_height as u32 / self.plot_config.char_height;
            let offset = start_line as usize * 16;
            let bytes = (*binary_data).get(offset, line_count as usize * 16);
            let mut elements = VecDeque::new();
            match bytes {
                Some(bytes) => {
                    let element = Element::Rectangle(RectangleElement { x: 0, y: 0, width: self.plot_config.width() as i32, height: view_height as i32, bg: (255, 255, 255) });
                    elements.push_back(element);
                    
                    let element = Element::Rectangle(RectangleElement { x: 0, y: 0, width: self.plot_config.offset_view_width as i32, height: view_height as i32, bg: (224, 224, 224) });
                    elements.push_back(element);

                    for (line, line_offset) in (0..bytes.len()).step_by(16).enumerate() {
                        let text = format!("{:08X}", line_offset);
                        let y = line * self.plot_config.char_height as usize;
                        let element = Element::Byte(TextElement { text, x: 0, y: y as i32, fg: (117, 117, 117) });
                        elements.push_back(element);
                    }
    
                    for (i, byte) in bytes.iter().enumerate() {
                        let line = i / 16;
                        let index = i % 16;
                        let text = format!("{:02X}", byte);
                        let mut x = self.plot_config.offset_view_width + (self.plot_config.char_width + self.plot_config.hex_byte_width) * index as u32;
                        if index >= 8 {
                            x += self.plot_config.char_width;
                        }
                        let y = line as u32 * self.plot_config.char_height;

                        let element = Element::Byte(TextElement { text, x: x as i32, y: y as i32, fg: (0, 0, 0) });
                        elements.push_back(element);
                    }

                    for (i, byte) in bytes.iter().enumerate() {
                        let line = i / 16;
                        let index = i % 16;
                        let text = {
                            let c = match char::from_u32(*byte as u32) {
                                Some(c) => if c.is_ascii_graphic() { c } else { '.' },
                                None => '.',
                            };
                            format!("{}", c)
                        };
                        let x = self.plot_config.offset_view_width + self.plot_config.hex_view_width() + index as u32 * self.plot_config.char_width;
                        let y = line as u32 * self.plot_config.char_height;

                        let element = Element::Byte(TextElement { text, x: x as i32, y: y as i32, fg: (0, 0, 0) });
                        elements.push_back(element);
                    }
                },
                None => todo!(),
            };
            return Some(View {
                    offset,
                    elements,
                    ..Default::default()
                });
        }
        None
    }
}
