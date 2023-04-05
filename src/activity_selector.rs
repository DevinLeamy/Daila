#![allow(dead_code)]
use tui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Widget},
};

pub trait ActivitySelectorValue {
    fn name(&self) -> &str;
    fn completed(&self) -> bool;
}

pub struct ActivitySelector<'a, T: ActivitySelectorValue> {
    values: Vec<&'a T>,
}

impl<'a, T: ActivitySelectorValue> Default for ActivitySelector<'a, T> {
    fn default() -> Self {
        Self { values: vec![] }
    }
}

impl<'a, T: ActivitySelectorValue> ActivitySelector<'a, T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn values(mut self, values: Vec<&'a T>) -> Self {
        self.values = values;
        self
    }
}

impl<'a, T: ActivitySelectorValue> Widget for ActivitySelector<'a, T> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Activity Selector");
        block.render(area, buf);
    }
}
