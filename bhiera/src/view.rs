use std::collections::VecDeque;

use crate::Element;

#[derive(Default)]
pub struct View {
    elements: VecDeque<Element>,
    cursors: Vec<(u32, u32, u32, u32)>,
}

impl View {
    pub fn new(elements: VecDeque<Element>, cursors: Vec<(u32, u32, u32, u32)>) -> Self {
        Self {
            elements,
            cursors,
            ..Default::default()
        }
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
