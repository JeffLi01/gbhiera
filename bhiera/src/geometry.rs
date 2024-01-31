use std::{
    cmp::{max, min},
    collections::VecDeque,
};

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
    /* below values are calculated */
    hex_view_start: u32,
    hex_view_width: u32,
    hex_view_end: u32,
    char_view_start: u32,
    char_view_width: u32,
    char_view_end: u32,
    width: u32,
}

impl Geometry {
    pub fn new(
        char_width: u32,
        char_height: u32,
        hex_byte_width: u32,
        offset_view_width: u32,
    ) -> Self {
        let hex_view_start = offset_view_width;
        let hex_view_width = (char_width + hex_byte_width) * 16;
        let hex_view_end = hex_view_start + hex_view_width;
        let char_view_start = hex_view_end + char_width * 2;
        let char_view_width = char_width * 16;
        let char_view_end = char_view_start + char_view_width;
        let width = char_view_end + char_width;
        Self {
            char_width,
            char_height,
            hex_byte_width,
            offset_view_width,
            hex_view_start,
            hex_view_width,
            hex_view_end,
            char_view_start,
            char_view_width,
            char_view_end,
            width,
        }
    }

    pub fn height(&self, byte_count: usize) -> u32 {
        let total_line_count = (byte_count + 15) / 16;
        self.char_height * total_line_count as u32
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn byte_offset(&self, view_start: u32) -> usize {
        let start_line = self.line_index(view_start);
        start_line * 16
    }

    fn line_index(&self, view_start: u32) -> usize {
        ((view_start + self.char_height - 1) / self.char_height) as usize
    }

    pub fn line_count(&self, view_height: u32) -> usize {
        (view_height / self.char_height) as usize
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
        let byte_offset = self.byte_offset(view_start);
        let line_count = self.line_count(view_height);
        let mut cursors = Vec::new();
        if current_byte >= byte_offset && current_byte < (byte_offset + line_count * 16) {
            let cursor_width = 2;
            let cursor_height = self.char_height;
            let line_index = (current_byte - byte_offset) / 16;
            let byte_index = (current_byte - byte_offset) % 16;
            let combo_width = self.char_width + self.hex_byte_width;
            let x = self.hex_view_start
                + if byte_index < 8 {
                    byte_index as u32 * combo_width
                } else {
                    byte_index as u32 * combo_width + self.char_width
                };
            let y: u32 = { line_index as u32 * self.char_height };
            cursors.push((x, y, cursor_width, cursor_height));
            let x = self.char_view_start + byte_index as u32 * self.char_width;
            cursors.push((x, y, cursor_width, cursor_height));
        }
        cursors
    }

    pub fn coordinate_to_byte(&self, y_offset: u32, x: u32, y: u32) -> usize {
        let line_index = (y_offset + y) / self.char_height;
        let combo_width = self.char_width + self.hex_byte_width;
        let byte_index = match x {
            x if x < self.hex_view_start => 0,
            x if x < self.hex_view_end => (x - self.hex_view_start) / combo_width,
            x if x < (self.hex_view_end + self.char_view_start) / 2 => 15,
            x if x < self.char_view_start + self.char_width / 2 => 0,
            x if x < self.width() - self.char_width => {
                (x - self.char_view_start + self.char_width / 2) / self.char_width
            }
            _ => 15,
        };
        (line_index * 16 + byte_index) as usize
    }

    fn hex_coordinate(&self, byte_offset: usize) -> (u32, u32, u32, u32) {
        let x = {
            let byte_index = byte_offset % 16;
            self.hex_view_start
                + byte_index as u32 * (self.hex_byte_width + self.char_width)
                + if byte_index < 8 { 0 } else { self.char_width }
        };
        let y = (byte_offset / 16) as u32 * self.char_height;
        (x, y, self.hex_byte_width, self.char_height)
    }

    fn char_coordinate(&self, byte_offset: usize) -> (u32, u32, u32, u32) {
        let x = self.char_view_start + (byte_offset % 16) as u32 * self.char_width;
        let y = (byte_offset / 16) as u32 * self.char_height;
        (x, y, self.char_width, self.char_height)
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

    pub fn selection(
        &self,
        view_start: u32,
        view_height: u32,
        selection_begin: usize,
        selection_end: usize,
    ) -> VecDeque<Element> {
        let mut elements = VecDeque::new();
        if selection_begin == selection_end {
            return elements;
        }
        let (selection_begin, selection_end) = if selection_begin <= selection_end {
            (selection_begin, selection_end)
        } else {
            (selection_end, selection_begin)
        };
        let byte_offset = self.byte_offset(view_start);
        let capacity = self.line_count(view_height) * 16;
        let visible_begin = max(selection_begin, byte_offset);
        let visible_end = min(selection_end, byte_offset + capacity);
        if visible_begin >= visible_end {
            return elements;
        }
        let (x1, y1, _, _) = self.hex_coordinate(visible_begin - byte_offset);
        let (x2, y2, width, _) = self.hex_coordinate(visible_end - byte_offset - 1);
        if y2 == y1 {
            let element = Element::Rectangle(RectangleElement {
                x: x1 as i32,
                y: y1 as i32,
                width: (x2 + width - x1) as i32,
                height: self.char_height as i32,
                bg: (0, 220, 220),
            });
            elements.push_back(element);
        } else if y2 > y1 {
            let element = Element::Rectangle(RectangleElement {
                x: x1 as i32,
                y: y1 as i32,
                width: (self.hex_view_end - x1) as i32,
                height: self.char_height as i32,
                bg: (0, 220, 220),
            });
            elements.push_back(element);
            let element = Element::Rectangle(RectangleElement {
                x: self.hex_view_start as i32,
                y: (y1 + self.char_height) as i32,
                width: self.hex_view_width as i32,
                height: (y2 - y1 - self.char_height) as i32,
                bg: (0, 220, 220),
            });
            elements.push_back(element);
            let element = Element::Rectangle(RectangleElement {
                x: self.hex_view_start as i32,
                y: y2 as i32,
                width: (x2 + width - self.hex_view_start) as i32,
                height: self.char_height as i32,
                bg: (0, 220, 220),
            });
            elements.push_back(element);
        }
        let (x1, y1, _, _) = self.char_coordinate(visible_begin - byte_offset);
        let (x2, y2, width, _) = self.char_coordinate(visible_end - byte_offset - 1);
        println!("x1: {x1}, y1: {y1}");
        println!("x2: {x2}, y2: {y2}, width: {width}");
        if y2 == y1 {
            let element = Element::Rectangle(RectangleElement {
                x: x1 as i32,
                y: y1 as i32,
                width: (x2 + width - x1) as i32,
                height: self.char_height as i32,
                bg: (0, 220, 220),
            });
            elements.push_back(element);
        } else if y2 > y1 {
            let element = Element::Rectangle(RectangleElement {
                x: x1 as i32,
                y: y1 as i32,
                width: (self.char_view_end - x1) as i32,
                height: self.char_height as i32,
                bg: (0, 220, 220),
            });
            elements.push_back(element);
            let element = Element::Rectangle(RectangleElement {
                x: self.char_view_start as i32,
                y: (y1 + self.char_height) as i32,
                width: self.char_view_width as i32,
                height: (y2 - y1 - self.char_height) as i32,
                bg: (0, 220, 220),
            });
            elements.push_back(element);
            let element = Element::Rectangle(RectangleElement {
                x: self.char_view_start as i32,
                y: y2 as i32,
                width: (x2 + width - self.char_view_start) as i32,
                height: self.char_height as i32,
                bg: (0, 220, 220),
            });
            elements.push_back(element);
        }
        elements
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
                self.hex_view_start + (self.char_width + self.hex_byte_width) * index as u32;
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
            let x = self.char_view_start + index as u32 * self.char_width;
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
