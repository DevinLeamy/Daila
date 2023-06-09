use crossterm::event::{Event, KeyCode};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, StatefulWidget, Widget},
};

use crate::{activites::ActivityId, popup::Popup};

#[derive(Default)]
pub struct ActivityPopup {}

pub enum ActivityPopupAction {
    CreateActivity(String),
    EditActivity(ActivityId, String),
    Exit,
}

#[derive(Copy, Clone)]
enum CursorPosition {
    TextInput,
    CreateOrEditButton,
    ExitButton,
}

impl CursorPosition {
    fn next(&self, last_position: Option<CursorPosition>, direction: KeyCode) -> Self {
        match &self {
            CursorPosition::TextInput => match direction {
                KeyCode::Down => {
                    if last_position.is_some()
                        && !matches!(last_position.unwrap(), CursorPosition::TextInput)
                    {
                        last_position.unwrap()
                    } else {
                        CursorPosition::ExitButton
                    }
                }
                _ => CursorPosition::TextInput,
            },
            CursorPosition::CreateOrEditButton => match direction {
                KeyCode::Up => CursorPosition::TextInput,
                KeyCode::Left => CursorPosition::ExitButton,
                _ => CursorPosition::CreateOrEditButton,
            },
            CursorPosition::ExitButton => match direction {
                KeyCode::Up => CursorPosition::TextInput,
                KeyCode::Right => CursorPosition::CreateOrEditButton,
                _ => CursorPosition::ExitButton,
            },
        }
    }
}

enum PopupType {
    Create,
    Edit,
}

/**
 * State for an activity editor or creator popup.
 */
pub struct ActivityPopupState {
    last_cursor_position: Option<CursorPosition>,
    cursor_position: CursorPosition,
    text_input: String,
    popup_type: PopupType,
    activity_id: Option<ActivityId>,
}

impl ActivityPopupState {
    /**
     * Initialize state for an activity editor popup.
     */
    pub fn new_editor(activity_title: String, activity_id: ActivityId) -> Self {
        Self {
            last_cursor_position: None,
            cursor_position: CursorPosition::TextInput,
            text_input: activity_title,
            popup_type: PopupType::Edit,
            activity_id: Some(activity_id),
        }
    }

    /**
     * Initialize state for an activity creator popup.
     */
    pub fn new_creator() -> Self {
        Self {
            last_cursor_position: None,
            cursor_position: CursorPosition::TextInput,
            text_input: String::new(),
            popup_type: PopupType::Create,
            activity_id: None,
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
                    CursorPosition::CreateOrEditButton => match state.popup_type {
                        PopupType::Create => Some(ActivityPopupAction::CreateActivity(
                            state.text_input.clone(),
                        )),
                        PopupType::Edit => Some(ActivityPopupAction::EditActivity(
                            state.activity_id.unwrap(),
                            state.text_input.clone(),
                        )),
                    },
                    CursorPosition::ExitButton => Some(ActivityPopupAction::Exit),
                },
                KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down => {
                    let new_position = state
                        .cursor_position
                        .next(state.last_cursor_position, key_event.code);
                    state.last_cursor_position = Some(state.cursor_position);
                    state.cursor_position = new_position;
                    None
                }
                KeyCode::Char(c) if matches!(state.cursor_position, CursorPosition::TextInput) => {
                    state.text_input.push(c);
                    None
                }
                KeyCode::Backspace => {
                    state.text_input.pop();
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

    fn render(self, area: Rect, buffer: &mut Buffer, state: &mut Self::State) {
        let block = Block::default()
            .title("   Activity Editor   ")
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center)
            .style(Style::default());

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(area.height - 3),
                Constraint::Length(1),
            ])
            .split(area);

        let text_layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(vec![
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ])
            .split(layout[0]);

        let bottom_row = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(layout[1]);

        let text_input_title = Block::default()
            .title("(new activity name)")
            .borders(Borders::NONE)
            .title_alignment(Alignment::Center)
            .style(Style::default());

        let selected_color = Color::Black;
        let not_selected_color = Color::Gray;
        let text_input = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .style(Style::default().bg(
                if matches!(state.cursor_position, CursorPosition::TextInput) {
                    selected_color
                } else {
                    not_selected_color
                },
            ));

        let exit = Block::default()
            .title("exit")
            .borders(Borders::NONE)
            .title_alignment(Alignment::Center)
            .style(Style::default().bg(
                if matches!(state.cursor_position, CursorPosition::ExitButton) {
                    selected_color
                } else {
                    not_selected_color
                },
            ));
        let create = Block::default()
            .title(match state.popup_type {
                PopupType::Create => "create",
                PopupType::Edit => "save",
            })
            .borders(Borders::NONE)
            .title_alignment(Alignment::Center)
            .style(Style::default().bg(
                if matches!(state.cursor_position, CursorPosition::CreateOrEditButton) {
                    selected_color
                } else {
                    not_selected_color
                },
            ));

        let text = if state.text_input.len() == 0 {
            String::from("Enter activity name")
        } else {
            let mut temp = state.text_input.clone();
            if matches!(state.cursor_position, CursorPosition::TextInput) {
                temp.push_str("| ");
            } else {
                temp.push_str("  ");
            }
            temp
        };

        block.render(area, buffer);
        text_input.render(text_layout[1], buffer);
        text_input_title.render(text_layout[2], buffer);
        exit.render(bottom_row[0], buffer);
        create.render(bottom_row[1], buffer);

        for i in 0..text.len() {
            buffer
                .get_mut(text_layout[1].x + i as u16 + 1, text_layout[1].y + 1)
                .set_symbol(&text);
        }
    }
}
