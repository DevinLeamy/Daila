use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
};

pub struct ActivityPopup {}

impl Default for ActivityPopup {
    fn default() -> Self {
        ActivityPopup {}
    }
}

impl Widget for ActivityPopup {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let block = Block::default()
            .title("Content")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Blue));

        block.render(area, buffer)
    }
}
