use std::io;

use chrono::NaiveDate;
use crossterm::event::{self, Event, KeyCode};
use ratatui::backend::Backend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Terminal;

use crate::activites::{
    self, ActivitiesStore, Activity, ActivityId, ActivityOption, ActivityTypesStore,
};
use crate::activity_popup::{ActivityPopup, ActivityPopupAction, ActivityPopupState};
use crate::activity_selector::{ActivitySelector, ActivitySelectorState, ActivitySelectorValue};
use crate::confirmation_popup::{
    ConfirmationPopup, ConfirmationPopupAction, ConfirmationPopupState,
};
use crate::file::File;
use crate::heatmap::HeatMap;
use crate::popup::{self, Popup};

pub enum ConfirmationAction {
    SaveWithoutQuitting,
    DeleteActivity(ActivityId),
}

use DailaEvent::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum DailaEvent {
    GotoPreviousDay,
    GotoNextDay,
    GotoToday,
    ActivityUp,
    ActivityDown,
    ActivityLeft,
    ActivityRight,
    ToggleSelectedActivity,
    SaveAndQuit,
    QuitWithoutSaving,
    CreateNewActivity,
    EditSelectedActivity,
    DeleteSelectedActivity,
}

impl DailaEvent {
    fn from_event(event: &Event) -> Option<Self> {
        match event {
            Event::Key(key_event) => Self::from_keycode(key_event.code),
            _ => None,
        }
    }

    fn from_keycode(code: KeyCode) -> Option<Self> {
        match code {
            KeyCode::Char('f') => Some(GotoNextDay),
            KeyCode::Char('F') => Some(GotoPreviousDay),
            KeyCode::Char('r') => Some(GotoToday),
            KeyCode::Right => Some(ActivityRight),
            KeyCode::Left => Some(ActivityLeft),
            KeyCode::Up => Some(ActivityUp),
            KeyCode::Down => Some(ActivityDown),
            KeyCode::Char('s') => Some(SaveAndQuit),
            KeyCode::Char('q') => Some(QuitWithoutSaving),
            KeyCode::Char('c') => Some(CreateNewActivity),
            KeyCode::Char('e') => Some(EditSelectedActivity),
            KeyCode::Char('d') => Some(DeleteSelectedActivity),
            KeyCode::Char('a') => Some(ToggleSelectedActivity),
            _ => None,
        }
    }

    fn to_char(&self) -> char {
        match &self {
            GotoNextDay => 'f',
            GotoPreviousDay => 'F',
            GotoToday => 'r',
            ToggleSelectedActivity => 'a',
            SaveAndQuit => 's',
            QuitWithoutSaving => 'q',
            CreateNewActivity => 'c',
            EditSelectedActivity => 'e',
            DeleteSelectedActivity => 'd',
            _ => '_',
        }
    }

    fn to_description(&self) -> String {
        let description = match &self {
            GotoNextDay => "next day",
            GotoPreviousDay => "previous day",
            GotoToday => "today",
            ToggleSelectedActivity => "toggle selected activity",
            SaveAndQuit => "save and quit",
            QuitWithoutSaving => "quit without saving",
            CreateNewActivity => "add new activity type",
            EditSelectedActivity => "edit the selected activity type",
            DeleteSelectedActivity => "delete the selected activity type",
            _ => "unknown",
        };

        String::from(description)
    }
}

pub enum DailaState {
    Default,
    ActivityPopup {
        state: ActivityPopupState,
    },
    ConfirmationPopup {
        action: ConfirmationAction,
        state: ConfirmationPopupState,
    },
}

pub struct Daila {
    activity_types: ActivityTypesStore,
    activities: ActivitiesStore,
    // Date displayed in the activity selector.
    active_date: NaiveDate,
    activity_selector_state: ActivitySelectorState,
    running: bool,
    state: DailaState,
    // Refresh the display.
    refresh: bool,
}

impl Daila {
    pub fn new() -> Self {
        let activity_types = ActivityTypesStore::load();
        let activity_types_len = activity_types.len();

        Self {
            activity_types: activity_types,
            activities: ActivitiesStore::load(),
            active_date: chrono::Local::now().date_naive(),
            activity_selector_state: ActivitySelectorState::new(activity_types_len),
            running: false,
            state: DailaState::Default,
            refresh: false,
        }
    }

    pub fn instructions_block(&self) -> Paragraph {
        let instructions = vec![
            DailaEvent::GotoPreviousDay,
            DailaEvent::GotoNextDay,
            DailaEvent::GotoToday,
            DailaEvent::ToggleSelectedActivity,
            DailaEvent::CreateNewActivity,
            DailaEvent::EditSelectedActivity,
            DailaEvent::DeleteSelectedActivity,
            DailaEvent::SaveAndQuit,
            DailaEvent::QuitWithoutSaving,
        ];
        let strings: Vec<String> = instructions
            .into_iter()
            .map(|event| format!("{}: {}", event.to_char(), event.to_description()))
            .collect();
        let string = strings.join("\n");

        Paragraph::new(Text::raw(string.to_owned())).block(Block::default().borders(Borders::ALL))
    }

    fn parse_input_event(&self, event: &Event) -> Option<DailaEvent> {
        DailaEvent::from_event(event)
    }

    fn handle_event(&mut self, event: Result<Event, io::Error>) -> Option<()> {
        let event = event.unwrap();
        match self.state {
            DailaState::Default => {
                let daila_event = self.parse_input_event(&event)?;
                match daila_event {
                    QuitWithoutSaving => {
                        self.refresh = true;
                        self.state = DailaState::ConfirmationPopup {
                            action: ConfirmationAction::SaveWithoutQuitting,
                            state: ConfirmationPopupState::new(String::from(
                                "Quit without saving?",
                            )),
                        }
                    }
                    SaveAndQuit => {
                        self.running = false;
                        // Save any unsaved changes.
                        self.activity_types.save();
                        self.activities.save();
                    }
                    DailaEvent::ToggleSelectedActivity => {
                        // Toggle the activity.
                        if let Some(activity_option) = self.selected_activity_option() {
                            let activity = Activity::new(
                                activity_option.activity_id(),
                                self.active_date.clone(),
                            );
                            if activity_option.completed() {
                                self.activities.remove_activity(activity);
                            } else {
                                self.activities.add_activity(activity);
                            }
                        }
                    }
                    CreateNewActivity => {
                        self.refresh = true;
                        self.state = DailaState::ActivityPopup {
                            state: ActivityPopupState::new_creator(),
                        };
                    }
                    EditSelectedActivity => {
                        self.refresh = true;
                        if let Some(activity_option) = self.selected_activity_option() {
                            self.state = DailaState::ActivityPopup {
                                state: ActivityPopupState::new_editor(
                                    activity_option.name().to_owned(),
                                    activity_option.activity_id(),
                                ),
                            };
                        }
                    }
                    DeleteSelectedActivity => {
                        self.refresh = true;
                        if let Some(activity_option) = self.selected_activity_option() {
                            self.state = DailaState::ConfirmationPopup {
                                action: ConfirmationAction::DeleteActivity(
                                    activity_option.activity_id(),
                                ),
                                state: ConfirmationPopupState::new(format!(
                                    "Confirm deletion of: {}",
                                    activity_option.name()
                                )),
                            }
                        }
                    }
                    GotoPreviousDay => self.active_date = self.active_date.pred_opt().unwrap(),
                    GotoNextDay => self.active_date = self.active_date.succ_opt().unwrap(),
                    GotoToday => self.active_date = chrono::Local::now().date_naive(),
                    ActivityLeft => self.activity_selector_state.select_left(),
                    ActivityRight => self.activity_selector_state.select_right(),
                    ActivityUp => self.activity_selector_state.select_up(),
                    ActivityDown => self.activity_selector_state.select_down(),
                }
            }
            DailaState::ActivityPopup { ref mut state } => {
                let action = ActivityPopup::handle_event(&event, state)?;
                self.refresh = true;
                match action {
                    ActivityPopupAction::Exit => {
                        self.state = DailaState::Default;
                    }
                    ActivityPopupAction::CreateActivity(title) => {
                        self.state = DailaState::Default;
                        self.activity_types.create_new_activity(title);
                        self.activity_selector_state =
                            ActivitySelectorState::new(self.activity_types.activity_types().len());
                    }
                    ActivityPopupAction::EditActivity(title, id) => {
                        self.state = DailaState::Default;
                        self.activity_types.update_activity(id, title);
                    }
                }
            }
            DailaState::ConfirmationPopup {
                ref action,
                ref mut state,
            } => {
                let popup_action = ConfirmationPopup::handle_event(&event, state)?;
                match popup_action {
                    ConfirmationPopupAction::Accept => match action {
                        ConfirmationAction::SaveWithoutQuitting => {
                            // Quit, without saving
                            self.running = false;
                        }
                        ConfirmationAction::DeleteActivity(id) => {
                            self.activity_types.delete_activity_type(id);
                            self.activity_selector_state = ActivitySelectorState::new(
                                self.activity_types.activity_types().len(),
                            );
                        }
                    },
                    ConfirmationPopupAction::Decline => (),
                }
                self.state = DailaState::Default;
            }
        };

        Some(())
    }

    fn activity_selector_options(&self) -> Vec<ActivityOption> {
        activites::activity_options(
            &self.activity_types,
            &self.activities,
            self.active_date.clone(),
        )
    }

    fn selected_activity_option(&self) -> Option<ActivityOption> {
        if let Some(index) = self.activity_selector_state.selected_index() {
            self.activity_selector_options().get(index).cloned()
        } else {
            None
        }
    }

    fn heatmap_values(&self) -> Vec<&Activity> {
        if self.activity_selector_state.selected_index().is_none() {
            return vec![];
        }
        let activity_types = self.activity_types.activity_types();
        let selected_activity =
            activity_types[self.activity_selector_state.selected_index().unwrap()];
        self.activities.activities_with_type(&selected_activity)
    }

    pub fn run_daila<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), io::Error> {
        self.running = true;
        while self.running {
            if self.refresh {
                terminal.clear().unwrap();
                self.refresh = false;
            }
            terminal.draw(|frame| {
                let heatmap_values = self.heatmap_values();
                let heatmap = HeatMap::default().values(heatmap_values);
                let selector_options = self.activity_selector_options();
                let frame_size = frame.size();
                let selector = ActivitySelector::<ActivityOption>::default()
                    .values(selector_options.iter().map(|o| o).collect())
                    .title(self.active_date.format("%A, %-d %B, %C%y").to_string());

                let display_size = Rect {
                    x: frame_size.x,
                    y: frame_size.y,
                    width: heatmap.width(),
                    height: frame_size.height,
                };

                let required_height = selector.height() + heatmap.height();
                let required_width = heatmap.width();
                if required_height > frame_size.height || required_width > frame_size.width {
                    // Display notice to make the terminal bigger.
                    let notice_block = Block::default()
                        .title("  Make the terminal larger  ")
                        .title_alignment(Alignment::Center)
                        .style(Style::default().fg(Color::Red))
                        .border_type(BorderType::Rounded)
                        .borders(Borders::ALL);
                    frame.render_widget(notice_block, display_size);
                    return;
                }

                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Length(selector.height()),
                            Constraint::Length(heatmap.height()),
                            Constraint::Length(10),
                        ]
                        .as_ref(),
                    )
                    .split(display_size.clone());

                frame.render_widget(heatmap, chunks[1]);
                frame.render_widget(self.instructions_block(), chunks[2]);
                frame.render_stateful_widget(
                    selector,
                    chunks[0],
                    &mut self.activity_selector_state,
                );

                match &mut self.state {
                    DailaState::ActivityPopup { ref mut state } => popup::render_in_frame(
                        frame,
                        &display_size,
                        50,
                        70,
                        ActivityPopup::default(),
                        state,
                    ),
                    DailaState::ConfirmationPopup {
                        action: _action,
                        ref mut state,
                    } => popup::render_in_frame(
                        frame,
                        &display_size,
                        50,
                        70,
                        ConfirmationPopup::default(),
                        state,
                    ),
                    _ => (),
                }
            })?;
            self.handle_event(event::read());
        }

        Ok(())
    }
}
