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

    pub fn cursors(&self) -> std::slice::Iter<'_, (u32, u32, u32, u32)> {
        self.cursors.iter()
    }

    pub fn elements(&self) -> std::collections::vec_deque::Iter<'_, Element> {
        self.elements.iter()
    }
}
