#![allow(dead_code)]
use std::cmp::min;

use tui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Widget},
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
    selected: Option<String>,
}

impl<'a, T: ActivitySelectorValue> Default for ActivitySelector<'a, T> {
    fn default() -> Self {
        Self {
            title: String::from("Activity Selector"),
            values: vec![],
            row_height: 5,
            values_per_row: 3,
            selected: None,
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

    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn selected_value(mut self, name: String) -> Self {
        self.selected = Some(name);
        self
    }

    fn render_value(&self, area: Rect, buffer: &mut Buffer, index: usize) {
        let item = self.values[index];
        let name = item.name();
        let (display_string, color) = if item.completed() {
            (format!("✅ {}: {}", index + 1, name), Color::Green)
        } else {
            (format!("―  {}: {}", index + 1, name), Color::White)
        };
        let borders = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        for j in 0..min(display_string.len(), area.width as usize) {
            buffer
                .get_mut(area.x + j as u16 + 2, area.y + 1)
                .set_fg(color)
                .set_symbol(&display_string);
        }

        if item.name() == &self.selected.clone().unwrap_or_default() {
            borders.render(area, buffer);
        }
    }

    fn formatted_title(&self) -> String {
        format!("{: ^width$}", self.title, width = 30)
    }

    pub fn height(&self) -> u16 {
        let values = self.values.len() as u16;
        let rows = if values % self.values_per_row != 0 {
            values / self.values_per_row + 1
        } else {
            values / self.values_per_row
        };
        // +2: Upper and lower border.
        rows * self.row_height + 2
    }
}

impl<'a, T: ActivitySelectorValue> Widget for ActivitySelector<'a, T> {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let title_style = Style::default().fg(Color::Yellow);
        let title = Span::styled(self.formatted_title(), title_style);

        let border = Block::default()
            .borders(Borders::ALL)
            .title(title)
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);
        let row_layout = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(vec![
                Constraint::Ratio(1, self.values_per_row as u32);
                self.values_per_row as usize
            ]);

        let mut row_cells: Vec<Rect> = vec![];
        for i in 0..self.values.len() {
            let row = i as u16 / self.values_per_row;
            if i as u16 % self.values_per_row == 0 {
                row_cells = row_layout.clone().split(Rect {
                    x: area.x,
                    y: area.y + self.row_height * row as u16,
                    width: area.width,
                    height: self.row_height,
                });
            }
            let grid_index = (i as u16 % self.values_per_row) as usize;
            self.render_value(row_cells[grid_index], buffer, i);
        }
        border.render(area, buffer);
    }
}
