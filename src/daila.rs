use std::io;

use chrono::NaiveDate;
use crossterm::event::{self, Event, KeyCode};
use ratatui::backend::Backend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Terminal;

use crate::activites::{self, ActivitiesStore, Activity, ActivityOption, ActivityTypesStore};
use crate::activity_popup::{ActivityPopup, ActivityPopupAction, ActivityPopupState};
use crate::activity_selector::{ActivitySelector, ActivitySelectorState, ActivitySelectorValue};
use crate::confirmation_popup::{
    ConfirmationPopup, ConfirmationPopupAction, ConfirmationPopupState,
};
use crate::file::File;
use crate::heatmap::HeatMap;
use crate::popup::{self, Popup};

use DailaEvent::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum DailaEvent {
    GotoPreviousDay,
    GotoNextDay,
    GotoToday,
    NextHeatmapActivity,
    PreviousHeatmapActivity,
    ToggleActivity(u32),
    SaveAndQuit,
    QuitWithoutSaving,
    CreateNewActivity,
    EditSelectedActivity,
}

impl DailaEvent {
    fn from_event(event: &Event) -> Option<Self> {
        match event {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Char('k') => Some(GotoNextDay),
                KeyCode::Char('j') => Some(GotoPreviousDay),
                KeyCode::Char('t') => Some(GotoToday),
                KeyCode::Char('m') => Some(NextHeatmapActivity),
                KeyCode::Char('n') => Some(PreviousHeatmapActivity),
                KeyCode::Char('s') => Some(SaveAndQuit),
                KeyCode::Char('q') => Some(QuitWithoutSaving),
                KeyCode::Char('c') => Some(CreateNewActivity),
                KeyCode::Char('e') => Some(EditSelectedActivity),
                KeyCode::Char(c) if c.is_digit(10) => {
                    Some(ToggleActivity(c.to_digit(10).unwrap() as u32))
                }
                _ => None,
            },
            _ => None,
        }
    }
    fn to_instruction(&self) -> String {
        let instruction = match &self {
            GotoNextDay => "k",
            GotoPreviousDay => "j",
            GotoToday => "t",
            NextHeatmapActivity => "m",
            PreviousHeatmapActivity => "n",
            ToggleActivity(_) => "%d",
            SaveAndQuit => "s",
            QuitWithoutSaving => "q",
            CreateNewActivity => "c",
            EditSelectedActivity => "e",
        };

        String::from(instruction)
    }

    fn to_description(&self) -> String {
        let description = match &self {
            GotoNextDay => "next day",
            GotoPreviousDay => "previous day",
            GotoToday => "today",
            NextHeatmapActivity => "next heatmap activity",
            PreviousHeatmapActivity => "previous heatmap activity",
            ToggleActivity(_) => "toggle activity",
            SaveAndQuit => "save and quit",
            QuitWithoutSaving => "quit without saving",
            CreateNewActivity => "add new activity type",
            EditSelectedActivity => "edit the selected activity type",
        };

        String::from(description)
    }
}

pub enum DailaState {
    Default,
    ActivityPopup { state: ActivityPopupState },
    ConfirmationPopup { state: ConfirmationPopupState },
}

pub struct Daila {
    activity_types: ActivityTypesStore,
    activities: ActivitiesStore,
    // Date displayed in the activity selector.
    active_date: NaiveDate,
    activity_selector_state: ActivitySelectorState,
    running: bool,
    state: DailaState,
}

impl Daila {
    pub fn new() -> Self {
        let mut activity_types = ActivityTypesStore::load();
        let activity_types_len = activity_types.len();

        if activity_types_len == 0 {
            // Create a default activity.
            activity_types.create_new_activity(String::from("🏞️  Meditate"));
        }

        Self {
            activity_types: activity_types,
            activities: ActivitiesStore::load(),
            active_date: chrono::Local::now().date_naive(),
            activity_selector_state: ActivitySelectorState::new(activity_types_len),
            running: false,
            state: DailaState::Default,
        }
    }

    pub fn instructions_block(&self) -> Paragraph {
        let instructions = vec![
            DailaEvent::GotoPreviousDay,
            DailaEvent::GotoNextDay,
            DailaEvent::GotoToday,
            DailaEvent::NextHeatmapActivity,
            DailaEvent::PreviousHeatmapActivity,
            DailaEvent::ToggleActivity(0),
            DailaEvent::CreateNewActivity,
            DailaEvent::EditSelectedActivity,
            DailaEvent::SaveAndQuit,
            DailaEvent::QuitWithoutSaving,
        ];
        let strings: Vec<String> = instructions
            .into_iter()
            .map(|event| format!("{}: {}", event.to_instruction(), event.to_description()))
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
                        self.state = DailaState::ConfirmationPopup {
                            state: ConfirmationPopupState::default(),
                        }
                    }
                    SaveAndQuit => {
                        self.running = false;
                        // Save any unsaved changes.
                        self.activity_types.save();
                        self.activities.save();
                    }
                    DailaEvent::ToggleActivity(index) => {
                        // Toggle the activity.
                        if let Some(option) =
                            self.activity_selector_options().get((index - 1) as usize)
                        {
                            let activity =
                                Activity::new(option.activity_id(), self.active_date.clone());
                            if option.completed() {
                                self.activities.remove_activity(activity);
                            } else {
                                self.activities.add_activity(activity);
                            }
                        }
                    }
                    CreateNewActivity => {
                        self.state = DailaState::ActivityPopup {
                            state: ActivityPopupState::new_creator(),
                        };
                    }
                    EditSelectedActivity => {
                        if let Some(activity_option) = self.selected_activity_option() {
                            self.state = DailaState::ActivityPopup {
                                state: ActivityPopupState::new_editor(
                                    activity_option.name().to_owned(),
                                    activity_option.activity_id(),
                                ),
                            };
                        }
                    }
                    GotoPreviousDay => self.active_date = self.active_date.pred_opt().unwrap(),
                    GotoNextDay => self.active_date = self.active_date.succ_opt().unwrap(),
                    GotoToday => self.active_date = chrono::Local::now().date_naive(),
                    NextHeatmapActivity => self.activity_selector_state.select_next(),
                    PreviousHeatmapActivity => self.activity_selector_state.select_previous(),
                }
            }
            DailaState::ActivityPopup { ref mut state } => {
                let action = ActivityPopup::handle_event(&event, state)?;
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
            DailaState::ConfirmationPopup { ref mut state } => {
                let action = ConfirmationPopup::handle_event(&event, state);
                match action {
                    Some(ConfirmationPopupAction::Accept) => self.state = DailaState::Default,
                    Some(ConfirmationPopupAction::Decline) => self.state = DailaState::Default,
                    None => (),
                }
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
        let activity_types = self.activity_types.activity_types();
        let selected_activity =
            activity_types[self.activity_selector_state.selected_index().unwrap()];
        self.activities.activities_with_type(&selected_activity)
    }

    pub fn run_daila<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), io::Error> {
        self.running = true;
        let mut flush = 0;
        while self.running {
            flush += 1;
            // Flush every 100 frames.
            // Flushing every frame causes the terminal to flicker.
            if flush == 100 {
                terminal.clear().unwrap();
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
                    frame.render_widget(notice_block, frame_size);
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
                    DailaState::ConfirmationPopup { ref mut state } => popup::render_in_frame(
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
