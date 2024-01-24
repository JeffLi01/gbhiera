use std::collections::VecDeque;

use crate::{
    element::{RectangleElement, TextElement},
    Element,
};

#[derive(Clone, Copy, Default)]
pub struct Geometry {
    char_width: u32,
    char_height: u32,
    hex_byte_width: u32,
    offset_view_width: u32,
}

impl Geometry {
    pub fn new(
        char_width: u32,
        char_height: u32,
        hex_byte_width: u32,
        offset_view_width: u32,
    ) -> Self {
        Self {
            char_width,
            char_height,
            hex_byte_width,
            offset_view_width,
        }
    }

    pub fn height(&self, byte_count: usize) -> u32 {
        let total_line_count = (byte_count + 15) / 16;
        self.char_height * total_line_count as u32
    }

    pub fn width(&self) -> u32 {
        let right_margin = self.char_width;
        self.offset_view_width + self.hex_view_width() + self.char_view_width() + right_margin
    }

    pub fn byte_offset(&self, view_start: u32) -> usize {
        let start_line =
            (view_start + self.char_height - 1) / self.char_height;
        start_line as usize * 16
    }

    pub fn line_count(&self, view_height: u32) -> usize {
        (view_height / self.char_height) as usize
    }

    pub fn hex_view_width(&self) -> u32 {
        (self.char_width + self.hex_byte_width) * 16 + self.char_width * 2
    }

    pub fn char_view_width(&self) -> u32 {
        self.char_width * 16
    }

    pub fn calc_cursor(
        &self,
        view_start: u32,
        view_height: u32,
        current_byte: usize,
    ) -> Vec<(u32, u32, u32, u32)> {
        let view_start = view_start - view_start % self.char_height;
        let mut cursors = Vec::new();
        let cursor_width = 2;
        let cursor_height = self.char_height;
        let line_index = current_byte / 16;
        let byte_index = current_byte % 16;
        let combo_width = self.char_width + self.hex_byte_width;
        let x = self.offset_view_width
            + if byte_index < 8 {
                byte_index as u32 * combo_width
            } else {
                byte_index as u32 * combo_width + self.char_width
            };
        let y: u32 = { line_index as u32 * self.char_height };
        if y > view_start && y < view_start + view_height {
            cursors.push((x, y - view_start, cursor_width, cursor_height));
            let x = self.offset_view_width
                + self.hex_view_width()
                + byte_index as u32 * self.char_width;
            cursors.push((x, y - view_start, cursor_width, cursor_height));
        }
        cursors
    }

    pub fn coordinate_to_byte(&self, y_offset: u32, x: u32, y: u32) -> usize {
        let line_index = (y_offset + y) / self.char_height;
        let combo_width = self.char_width + self.hex_byte_width;
        let byte_index = match x {
            x if x < self.offset_view_width => 0,
            x if x < self.offset_view_width + self.hex_view_width() - self.char_width * 2 => {
                (x - self.offset_view_width) / combo_width
            }
            x if x < self.offset_view_width + self.hex_view_width() - self.char_width => 15,
            x if x < self.offset_view_width + self.hex_view_width() - self.char_width / 2 => 0,
            x if x < self.width() - self.char_width => {
                (x - self.offset_view_width - self.hex_view_width() + self.char_width / 2)
                    / self.char_width
            }
            _ => 15,
        };
        (line_index * 16 + byte_index) as usize
    }

    pub fn bg(&self, height: u32) -> Element {
        Element::Rectangle(RectangleElement {
            x: 0,
            y: 0,
            width: self.width() as i32,
            height: height as i32,
            bg: (255, 255, 255),
        })
    }

    pub fn offset_view_bg(&self, height: u32) -> Element {
        Element::Rectangle(RectangleElement {
            x: 0,
            y: 0,
            width: self.offset_view_width as i32,
            height: height as i32,
            bg: (224, 224, 224),
        })
    }

    pub fn offsets(&self, offset: usize, size: usize) -> VecDeque<Element> {
        let mut elements = VecDeque::new();
        for (line, line_offset) in (0..size).step_by(16).enumerate() {
            let text = format!("{:08X}", offset + line_offset);
            let y = line * self.char_height as usize;
            let element = Element::Byte(TextElement {
                text,
                x: 0,
                y: y as i32,
                fg: (117, 117, 117),
            });
            elements.push_back(element);
        }
        elements
    }

    pub fn text(&self, bytes: &[u8]) -> VecDeque<Element> {
        let mut elements = VecDeque::new();
        for (i, byte) in bytes.iter().enumerate() {
            let line = i / 16;
            let index = i % 16;
            let text = format!("{:02X}", byte);
            let mut x =
                self.offset_view_width + (self.char_width + self.hex_byte_width) * index as u32;
            if index >= 8 {
                x += self.char_width;
            }
            let y = line as u32 * self.char_height;

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
            let x = self.offset_view_width + self.hex_view_width() + index as u32 * self.char_width;
            let y = line as u32 * self.char_height;

            let element = Element::Byte(TextElement {
                text,
                x: x as i32,
                y: y as i32,
                fg: (0, 0, 0),
            });
            elements.push_back(element);
        }
        elements
    }
}
