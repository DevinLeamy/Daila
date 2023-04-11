use crossterm::event::{Event, KeyCode};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    widgets::{Block, Borders, StatefulWidget, Widget},
};

use crate::{activites::ActivityId, popup::Popup};

#[derive(Default)]
pub struct ActivityPopup {}

pub enum ActivityPopupAction {
    Create(String),
    Edit(ActivityId, String),
    Exit,
}

enum CursorPosition {
    TextInput,
    CreateButton,
    ExitButton,
}

impl CursorPosition {
    fn next(&self, direction: KeyCode) -> Self {
        match &self {
            CursorPosition::TextInput => match direction {
                KeyCode::Down => CursorPosition::ExitButton,
                _ => CursorPosition::TextInput,
            },
            CursorPosition::CreateButton => match direction {
                KeyCode::Up => CursorPosition::TextInput,
                KeyCode::Left => CursorPosition::ExitButton,
                _ => CursorPosition::CreateButton,
            },
            CursorPosition::ExitButton => match direction {
                KeyCode::Up => CursorPosition::TextInput,
                KeyCode::Right => CursorPosition::CreateButton,
                _ => CursorPosition::ExitButton,
            },
        }
    }
}

pub struct ActivityPopupState {
    cursor_position: CursorPosition,
    text_input: String,
}

impl Default for ActivityPopupState {
    fn default() -> Self {
        Self {
            cursor_position: CursorPosition::TextInput,
            text_input: String::new(),
        }
    }
}

impl Popup<ActivityPopupState> for ActivityPopup {
    type Action = ActivityPopupAction;

    fn handle_event(event: &Event, state: &mut ActivityPopupState) -> Option<Self::Action> {
        match event {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Enter => match state.cursor_position {
                    CursorPosition::TextInput => None,
                    CursorPosition::CreateButton => {
                        Some(ActivityPopupAction::Create(state.text_input.clone()))
                    }
                    CursorPosition::ExitButton => Some(ActivityPopupAction::Exit),
                },
                KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down => {
                    state.cursor_position = state.cursor_position.next(key_event.code);
                    None
                }
                _ => None,
            },
            _ => None,
        }
    }
}

impl StatefulWidget for ActivityPopup {
    type State = ActivityPopupState;

    fn render(self, area: Rect, buffer: &mut Buffer, _state: &mut Self::State) {
        let block = Block::default()
            .title("Activity Editor")
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center)
            .style(Style::default());

        block.render(area, buffer)
    }
}
