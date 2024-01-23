use std::collections::VecDeque;

use crate::{DataProvider, Element, Geometry};

#[derive(Default)]
pub struct Bhiera {
    data_provider: Option<Box<dyn DataProvider>>,
    geometry: Geometry,
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

    pub fn set_geometry(&mut self, geometry: &Geometry) {
        self.geometry = *geometry;
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
                (view_start + self.geometry.char_height - 1) / self.geometry.char_height;
            let line_count = view_height as u32 / self.geometry.char_height;
            let offset = start_line as usize * 16;
            let mut elements = VecDeque::new();
            if let Some(bytes) = (*binary_data).get(offset, line_count as usize * 16) {
                elements.push_back(self.geometry.bg(view_height));

                elements.push_back(self.geometry.offset_view_bg(view_height));

                elements.append(&mut self.geometry.offsets(offset, bytes.len()));

                elements.append(&mut self.geometry.text(bytes));
            };

            let cursors = self
                .geometry
                .calc_cursor(view_start, view_height, self.selection_end);

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
        self.selection_begin = self
            .geometry
            .coordinate_to_byte(self.view_y, x as u32, y as u32);
    }

    fn set_selection_end(&mut self, x: i32, y: i32) {
        self.selection_end = self
            .geometry
            .coordinate_to_byte(self.view_y, x as u32, y as u32);
    }
}
