#![allow(dead_code)]
use std::cmp::min;

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, StatefulWidget, Widget},
};

const ACTIVITIES_PER_ROW: u16 = 3;

#[derive(Clone)]
pub struct ActivitySelectorState {
    activity_count: usize,
    selected_index: Option<usize>,
}

impl ActivitySelectorState {
    pub fn new(activity_count: usize) -> Self {
        return Self {
            activity_count,
            selected_index: if activity_count == 0 { None } else { Some(0) },
        };
    }
    pub fn select_right(&mut self) {
        if let Some(index) = self.selected_index {
            self.selected_index = Some((index + 1) % self.activity_count);
        }
    }

    pub fn select_left(&mut self) {
        if let Some(index) = self.selected_index {
            self.selected_index = Some((index + self.activity_count - 1) % self.activity_count);
        }
    }

    pub fn select_up(&mut self) {
        if let Some(index) = self.selected_index {
            if index >= ACTIVITIES_PER_ROW as usize {
                self.selected_index = Some(index - ACTIVITIES_PER_ROW as usize);
            }
        }
    }

    pub fn select_down(&mut self) {
        if let Some(index) = self.selected_index {
            if (index + ACTIVITIES_PER_ROW as usize) < self.activity_count {
                self.selected_index = Some(index + ACTIVITIES_PER_ROW as usize);
            }
        }
    }

    pub fn selected(&self, index: usize) -> bool {
        self.selected_index == Some(index)
    }

    pub fn selected_index(&self) -> Option<usize> {
        self.selected_index
    }
}

pub trait ActivitySelectorValue {
    fn name(&self) -> &str;
    fn completed(&self) -> bool;
}

pub struct ActivitySelector<'a, T: ActivitySelectorValue> {
    title: String,
    values: Vec<&'a T>,
    row_height: u16,
}

impl<'a, T: ActivitySelectorValue> Default for ActivitySelector<'a, T> {
    fn default() -> Self {
        Self {
            title: String::from("Activity Selector"),
            values: vec![],
            row_height: 5,
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

    fn render_value(&self, area: Rect, buffer: &mut Buffer, index: usize, selected: bool) {
        let item = self.values[index];
        let name = item.name();
        let (display_string, color) = if item.completed() {
            (format!("✅ {}", name), Color::Green)
        } else {
            (format!("―  {}", name), Color::White)
        };
        for j in 0..min(display_string.len(), area.width as usize) {
            buffer
                .get_mut(area.x + j as u16 + 2, area.y + 1)
                .set_fg(color)
                .set_symbol(&display_string);
        }

        if selected {
            // Draw borders around the selected item.
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .render(area, buffer);
        }
    }

    fn formatted_title(&self) -> String {
        format!("{: ^width$}", self.title, width = 34)
    }

    pub fn height(&self) -> u16 {
        let values = self.values.len() as u16;
        let rows = if values % ACTIVITIES_PER_ROW != 0 {
            values / ACTIVITIES_PER_ROW + 1
        } else {
            values / ACTIVITIES_PER_ROW
        };
        // +2: Upper and lower border.
        rows * self.row_height + 2
    }
}

impl<'a, T: ActivitySelectorValue> StatefulWidget for ActivitySelector<'a, T> {
    type State = ActivitySelectorState;

    fn render(self, area: Rect, buffer: &mut Buffer, state: &mut Self::State) {
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
                Constraint::Ratio(1, ACTIVITIES_PER_ROW as u32);
                ACTIVITIES_PER_ROW as usize
            ]);

        let mut row_cells: Vec<Rect> = vec![];
        for i in 0..self.values.len() {
            let row = i as u16 / ACTIVITIES_PER_ROW;
            if i as u16 % ACTIVITIES_PER_ROW == 0 {
                row_cells = row_layout
                    .clone()
                    .split(Rect {
                        x: area.x,
                        y: area.y + self.row_height * row as u16,
                        width: area.width,
                        height: self.row_height,
                    })
                    .to_vec();
            }
            let grid_index = (i as u16 % ACTIVITIES_PER_ROW) as usize;
            self.render_value(row_cells[grid_index], buffer, i, state.selected(i));
        }
        border.render(area, buffer);
    }
}
