use crossterm::event::{Event, KeyCode};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, StatefulWidget, Widget},
};

use crate::popup::{self, Popup};

#[derive(Copy, Clone)]
enum CursorPosition {
    LeftButton,
    RightButton,
}

impl CursorPosition {
    fn next(&self, direction: KeyCode) -> Self {
        match &self {
            CursorPosition::LeftButton => match direction {
                KeyCode::Right => CursorPosition::RightButton,
                _ => CursorPosition::LeftButton,
            },
            CursorPosition::RightButton => match direction {
                KeyCode::Left => CursorPosition::LeftButton,
                _ => CursorPosition::RightButton,
            },
        }
    }
}

#[derive(Default)]
pub struct ConfirmationPopup {}

pub enum ConfirmationPopupAction {
    Accept,
    Decline,
}

pub struct ConfirmationPopupState {
    cursor_position: CursorPosition,
    prompt: String,
}

impl ConfirmationPopupState {
    pub fn new(prompt: String) -> Self {
        Self {
            cursor_position: CursorPosition::LeftButton,
            prompt,
        }
    }
}

impl Popup<ConfirmationPopupState> for ConfirmationPopup {
    type Action = ConfirmationPopupAction;

    fn handle_event(event: &Event, state: &mut ConfirmationPopupState) -> Option<Self::Action> {
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Left | KeyCode::Right => {
                    state.cursor_position = state.cursor_position.next(key_event.code);
                    None
                }
                KeyCode::Enter => match state.cursor_position {
                    CursorPosition::LeftButton => Some(ConfirmationPopupAction::Decline),
                    CursorPosition::RightButton => Some(ConfirmationPopupAction::Accept),
                },
                _ => None,
            }
        } else {
            None
        }
    }
}

impl StatefulWidget for ConfirmationPopup {
    type State = ConfirmationPopupState;

    fn render(self, area: Rect, buffer: &mut Buffer, state: &mut Self::State) {
        let selected_color = Color::Black;
        let not_selected_color = Color::Gray;

        let block = Block::default()
            .title("  Confirmation  ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title_alignment(Alignment::Center)
            .style(Style::default());

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(area.height - 3),
                Constraint::Length(1),
            ])
            .split(area);

        let prompt = Block::default()
            .title(state.prompt.as_str())
            .title_alignment(Alignment::Center)
            .borders(Borders::NONE)
            .style(Style::default().fg(Color::Red));

        let bottom_row = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(layout[1]);

        let cancel = Block::default()
            .title(match state.cursor_position {
                CursorPosition::LeftButton => "[cancel]",
                _ => "cancel",
            })
            .borders(Borders::NONE)
            .title_alignment(Alignment::Center)
            .style(Style::default().bg(match state.cursor_position {
                CursorPosition::LeftButton => selected_color,
                _ => not_selected_color,
            }));
        let accept = Block::default()
            .title(match state.cursor_position {
                CursorPosition::RightButton => "[continue]",
                _ => "continue",
            })
            .borders(Borders::NONE)
            .title_alignment(Alignment::Center)
            .style(Style::default().bg(match state.cursor_position {
                CursorPosition::RightButton => selected_color,
                _ => not_selected_color,
            }));

        block.render(area, buffer);
        prompt.render(popup::centered_area(&layout[0], 90, 50), buffer);
        cancel.render(bottom_row[0], buffer);
        accept.render(bottom_row[1], buffer);
    }
}
