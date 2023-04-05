#![allow(dead_code)]
use std::cmp::min;

use tui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Widget,
};

pub trait ActivitySelectorValue {
    fn name(&self) -> &str;
    fn completed(&self) -> bool;
}

pub struct ActivitySelector<'a, T: ActivitySelectorValue> {
    title: String,
    values: Vec<&'a T>,
    row_height: u16,
    values_per_row: u16,
}

impl<'a, T: ActivitySelectorValue> Default for ActivitySelector<'a, T> {
    fn default() -> Self {
        Self {
            title: String::from("Activity Selector"),
            values: vec![],
            row_height: 4,
            values_per_row: 3,
        }
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

    fn render_value(&self, area: Rect, buffer: &mut Buffer, index: usize) {
        let item = self.values[index];
        let name = item.name();
        let display_string = if item.completed() {
            format!("✅ {}: {}", index + 1, name)
        } else {
            format!("―  {}: {}", index + 1, name)
        };

        for j in 0..min(display_string.len(), area.width as usize) {
            buffer
                .get_mut(area.x + j as u16, area.y)
                .set_symbol(&display_string);
        }
    }
}

impl<'a, T: ActivitySelectorValue> Widget for ActivitySelector<'a, T> {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let row_layout = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(vec![
                Constraint::Ratio(1, self.values_per_row as u32);
                self.values_per_row as usize
            ]);

        let row_height = 4;
        let mut row_cells = row_layout.clone().split(Rect {
            x: area.x,
            y: area.y,
            width: area.width,
            height: row_height,
        });
        for i in 0..self.values.len() {
            let row = i as u16 / self.values_per_row;
            if i as u16 % self.values_per_row == 0 {
                row_cells = row_layout.clone().split(Rect {
                    x: area.x,
                    y: area.y + row_height * row as u16,
                    width: area.width,
                    height: row_height,
                });
            }
            let grid_index = (i as u16 % self.values_per_row) as usize;
            self.render_value(row_cells[grid_index], buffer, i);
        }
    }
}
