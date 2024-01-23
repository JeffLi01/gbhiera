use std::collections::VecDeque;

use crate::{Geometry, DataProvider};

#[derive(Default)]
pub struct Bhiera {
    data_provider: Option<Box<dyn DataProvider>>,
    plot_config: Geometry,
    view_y: u32,
    selection_begin: usize,
    selection_end: usize,
}

impl Bhiera {
    pub fn new() -> Self {
        Self {
            data_provider: None,
            ..Default::default()
        }
    }

    pub fn set_geometry(&mut self, config: &Geometry) {
        self.plot_config = *config;
    }

    fn calc_cursor(&self, view_start: u32, view_height: u32) -> Vec<(u32, u32, u32, u32)> {
        let view_start = view_start - view_start % self.plot_config.char_height;
        let mut cursors = Vec::new();
        let cursor_width = 2;
        let cursor_height = self.plot_config.char_height;
        let line_index = self.selection_end / 16;
        let byte_index = self.selection_end % 16;
        let combo_width = self.plot_config.char_width + self.plot_config.hex_byte_width;
        let x = self.plot_config.offset_view_width + if byte_index < 8 {
            byte_index as u32 * combo_width
        } else {
            byte_index as u32 * combo_width + self.plot_config.char_width
        };
        let y: u32 = {
            line_index as u32 * self.plot_config.char_height
        };
        if y > view_start && y < view_start + view_height {
            cursors.push((x, y - view_start, cursor_width, cursor_height));
            let x = self.plot_config.offset_view_width + self.plot_config.hex_view_width() + byte_index as u32 * self.plot_config.char_width;
            cursors.push((x, y - view_start, cursor_width, cursor_height));
        }
        cursors
    }

    fn coordinate_to_byte(&self, x: u32, y: u32) -> usize {
        let line_index = (self.view_y + y) / self.plot_config.char_height;
        let combo_width = self.plot_config.char_width + self.plot_config.hex_byte_width;
        let byte_index = match x {
            x if x < self.plot_config.offset_view_width => 0,
            x if x < self.plot_config.offset_view_width + self.plot_config.hex_view_width() - self.plot_config.char_width * 2 => {
                (x - self.plot_config.offset_view_width) / combo_width
            },
            x if x < self.plot_config.offset_view_width + self.plot_config.hex_view_width() - self.plot_config.char_width => 15,
            x if x < self.plot_config.offset_view_width + self.plot_config.hex_view_width() - self.plot_config.char_width / 2 => 0,
            x if x < self.plot_config.width() - self.plot_config.char_width => {
                (x - self.plot_config.offset_view_width - self.plot_config.hex_view_width() + self.plot_config.char_width / 2) / self.plot_config.char_width
            },
            _ => 15,
        };
        (line_index * 16 + byte_index) as usize
    }
}

#[derive(Default)]
pub struct View {
    offset: usize,
    elements: VecDeque<Element>,
    cursors: Vec<(u32, u32, u32, u32)>,
}

impl View {
    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn get_cursur(&self) -> Vec<(u32, u32, u32, u32)> {
        self.cursors.to_owned()
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
    fn set_view_y(&mut self, y: u32);
    fn set_selection_begin(&mut self, x: i32, y: i32);
    fn set_selection_end(&mut self, x: i32, y: i32);
}

impl Model for Bhiera {
    fn set_data_provider(&mut self, provider: impl DataProvider + 'static) {
        self.data_provider.replace(Box::new(provider));
    }

    fn get_view(&self, view_start: u32, view_height: u32) -> Option<View> {
        if let Some(binary_data) = &self.data_provider {
            let start_line =
                (view_start + self.plot_config.char_height - 1) / self.plot_config.char_height;
            let line_count = view_height as u32 / self.plot_config.char_height;
            let offset = start_line as usize * 16;
            let mut elements = VecDeque::new();
            if let Some(bytes) = (*binary_data).get(offset, line_count as usize * 16) {
                let element = Element::Rectangle(RectangleElement {
                    x: 0,
                    y: 0,
                    width: self.plot_config.width() as i32,
                    height: view_height as i32,
                    bg: (255, 255, 255),
                });
                elements.push_back(element);

                let element = Element::Rectangle(RectangleElement {
                    x: 0,
                    y: 0,
                    width: self.plot_config.offset_view_width as i32,
                    height: view_height as i32,
                    bg: (224, 224, 224),
                });
                elements.push_back(element);

                for (line, line_offset) in (0..bytes.len()).step_by(16).enumerate() {
                    let text = format!("{:08X}", offset + line_offset);
                    let y = line * self.plot_config.char_height as usize;
                    let element = Element::Byte(TextElement {
                        text,
                        x: 0,
                        y: y as i32,
                        fg: (117, 117, 117),
                    });
                    elements.push_back(element);
                }

                for (i, byte) in bytes.iter().enumerate() {
                    let line = i / 16;
                    let index = i % 16;
                    let text = format!("{:02X}", byte);
                    let mut x = self.plot_config.offset_view_width
                        + (self.plot_config.char_width + self.plot_config.hex_byte_width)
                            * index as u32;
                    if index >= 8 {
                        x += self.plot_config.char_width;
                    }
                    let y = line as u32 * self.plot_config.char_height;

                    let element = Element::Byte(TextElement {
                        text,
                        x: x as i32,
                        y: y as i32,
                        fg: (0, 0, 0),
                    });
                    elements.push_back(element);
                }

                for (i, byte) in bytes.iter().enumerate() {
                    let line = i / 16;
                    let index = i % 16;
                    let text = {
                        let c = match char::from_u32(*byte as u32) {
                            Some(c) => {
                                if c.is_ascii_graphic() {
                                    c
                                } else {
                                    '.'
                                }
                            }
                            None => '.',
                        };
                        format!("{}", c)
                    };
                    let x = self.plot_config.offset_view_width
                        + self.plot_config.hex_view_width()
                        + index as u32 * self.plot_config.char_width;
                    let y = line as u32 * self.plot_config.char_height;

                    let element = Element::Byte(TextElement {
                        text,
                        x: x as i32,
                        y: y as i32,
                        fg: (0, 0, 0),
                    });
                    elements.push_back(element);
                }
            };

            let cursors = self.calc_cursor(view_start, view_height);

            return Some(View {
                offset,
                elements,
                cursors,
                ..Default::default()
            });
        }
        None
    }

    fn set_view_y(&mut self, y: u32) {
        self.view_y = y;
    }

    fn set_selection_begin(&mut self, x: i32, y: i32) {
        self.selection_begin = self.coordinate_to_byte(x as u32, y as u32);
    }

    fn set_selection_end(&mut self, x: i32, y: i32) {
        self.selection_end = self.coordinate_to_byte(x as u32, y as u32);
    }
}
