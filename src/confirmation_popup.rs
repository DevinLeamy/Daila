use crossterm::event::{Event, KeyCode};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{Block, Borders, StatefulWidget, Widget},
};

use crate::popup::Popup;

#[derive(Default)]
pub struct ConfirmationPopup {}

pub enum ConfirmationPopupAction {
    Accept,
    Decline,
}

#[derive(Default)]
pub struct ConfirmationPopupState {}

impl Popup<ConfirmationPopupState> for ConfirmationPopup {
    type Action = ConfirmationPopupAction;

    fn handle_event(event: &Event, _state: &mut ConfirmationPopupState) -> Option<Self::Action> {
        match event {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Char('y') => Some(ConfirmationPopupAction::Accept),
                KeyCode::Char('n') => Some(ConfirmationPopupAction::Decline),
                _ => None,
            },
            _ => None,
        }
    }
}

impl StatefulWidget for ConfirmationPopup {
    type State = ConfirmationPopupState;

    fn render(self, area: Rect, buffer: &mut Buffer, _state: &mut Self::State) {
        let block = Block::default()
            .title("Content")
            .borders(Borders::ALL)
            .style(Style::default());

        block.render(area, buffer)
    }
}
