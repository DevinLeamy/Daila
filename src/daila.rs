use std::io;

use chrono::NaiveDate;
use crossterm::event::{self, Event, KeyCode};
use ratatui::backend::Backend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Terminal;

use crate::activites::{
    self, ActivitiesStore, Activity, ActivityOption, ActivityType, ActivityTypesStore,
};
use crate::activity_popup::ActivityPopup;
use crate::activity_selector::{ActivitySelector, ActivitySelectorValue};
use crate::file::File;
use crate::heatmap::HeatMap;

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
}

impl DailaEvent {
    fn from_event(event: Event) -> Option<Self> {
        match event {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Char('k') => Some(GotoNextDay),
                KeyCode::Char('j') => Some(GotoPreviousDay),
                KeyCode::Char('t') => Some(GotoToday),
                KeyCode::Char('m') => Some(NextHeatmapActivity),
                KeyCode::Char('n') => Some(PreviousHeatmapActivity),
                KeyCode::Char('s') => Some(SaveAndQuit),
                KeyCode::Char('q') => Some(QuitWithoutSaving),
                KeyCode::Char(c) => {
                    if c.is_digit(10) {
                        Some(ToggleActivity(c.to_digit(10).unwrap() as u32))
                    } else {
                        None
                    }
                }
                _ => None,
            },
            _ => None,
        }
    }
    fn to_instruction(&self) -> String {
        let instruction = match &self {
            Self::GotoNextDay => "k",
            Self::GotoPreviousDay => "j",
            Self::GotoToday => "t",
            Self::NextHeatmapActivity => "m",
            Self::PreviousHeatmapActivity => "n",
            Self::ToggleActivity(_) => "%d",
            Self::SaveAndQuit => "s",
            Self::QuitWithoutSaving => "q",
        };

        String::from(instruction)
    }

    fn to_description(&self) -> String {
        let description = match &self {
            Self::GotoNextDay => "next day",
            Self::GotoPreviousDay => "previous day",
            Self::GotoToday => "today",
            Self::NextHeatmapActivity => "next heatmap activity",
            Self::PreviousHeatmapActivity => "previous heatmap activity",
            Self::ToggleActivity(_) => "toggle activity",
            Self::SaveAndQuit => "save and quit",
            Self::QuitWithoutSaving => "quit without saving",
        };

        String::from(description)
    }
}

pub struct Daila {
    activity_types: ActivityTypesStore,
    activities: ActivitiesStore,
    // Date displayed in the activity selector.
    active_date: NaiveDate,
    // Activity type displayed in the heatmap.
    heatmap_activity_type: ActivityType,
}

impl Daila {
    pub fn new() -> Self {
        let mut activity_types = ActivityTypesStore::load();
        if activity_types.activity_types().len() == 0 {
            // Create a default activity.
            activity_types.create_new_activity(String::from("🏞️  Meditate"));
        }
        let mut first_type = activity_types.activity_types()[0].clone();
        for activity in activity_types.activity_types() {
            if activity.id < first_type.id {
                first_type = activity.clone()
            }
        }

        Self {
            activity_types: activity_types,
            activities: ActivitiesStore::load(),
            active_date: chrono::Local::now().date_naive(),
            heatmap_activity_type: first_type,
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

    fn parse_input_event(&self, event: Result<Event, io::Error>) -> Option<DailaEvent> {
        match event {
            Ok(event) => DailaEvent::from_event(event),
            Err(_) => None,
        }
    }

    pub fn run_daila<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), io::Error> {
        let mut running = true;
        let mut show_activity_pop = false;

        while running {
            let options = activites::activity_options(
                &self.activity_types,
                &self.activities,
                self.active_date.clone(),
            );
            terminal.draw(|frame| {
                let frame_size = frame.size();
                let heatmap = HeatMap::default().values(
                    self.activities
                        .activities_with_type(&self.heatmap_activity_type),
                );
                let selector = ActivitySelector::<ActivityOption>::default()
                    .values(options.iter().map(|o| o).collect())
                    .title(self.active_date.format("%A, %-d %B, %C%y").to_string())
                    .selected_value(self.heatmap_activity_type.name.clone());

                let display_size = Rect {
                    x: frame_size.x,
                    y: frame_size.y,
                    width: heatmap.width(),
                    height: frame_size.height,
                };
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

                frame.render_widget(selector, chunks[0]);
                frame.render_widget(heatmap, chunks[1]);
                frame.render_widget(self.instructions_block(), chunks[2]);

                if show_activity_pop {
                    let height_percentage = 50;
                    let width_percentage = 70;

                    // Center the popup inside the activity.
                    let popup_area = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(
                            [
                                Constraint::Percentage((100 - height_percentage) / 2),
                                Constraint::Percentage(height_percentage),
                                Constraint::Percentage((100 - height_percentage) / 2),
                            ]
                            .as_ref(),
                        )
                        .split(display_size);

                    let popup_area = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [
                                Constraint::Percentage((100 - width_percentage) / 2),
                                Constraint::Percentage(width_percentage),
                                Constraint::Percentage((100 - width_percentage) / 2),
                            ]
                            .as_ref(),
                        )
                        .split(popup_area[1])[1];

                    frame.render_widget(Clear, popup_area);
                    frame.render_widget(ActivityPopup::default(), popup_area);
                }
            })?;
            let event = self.parse_input_event(event::read());
            if event.is_none() {
                continue;
            }
            match event.unwrap() {
                QuitWithoutSaving => running = false,
                SaveAndQuit => {
                    running = false;
                    // Save any unsaved changes.
                    self.activity_types.save();
                    self.activities.save();
                }
                DailaEvent::ToggleActivity(index) => {
                    // Toggle the activity.
                    if let Some(option) = options.get((index - 1) as usize) {
                        let activity =
                            Activity::new(option.activity_id(), self.active_date.clone());
                        if option.completed() {
                            // Toggle off.
                            self.activities.remove_activity(activity);
                        } else {
                            // Toggle on.
                            self.activities.add_activity(activity);
                        }
                    }
                }
                // KeyCode::Char('p') => show_activity_pop = !show_activity_pop,
                GotoPreviousDay => self.active_date = self.active_date.pred_opt().unwrap(),
                GotoNextDay => self.active_date = self.active_date.succ_opt().unwrap(),
                GotoToday => self.active_date = chrono::Local::now().date_naive(),
                DailaEvent::NextHeatmapActivity | DailaEvent::PreviousHeatmapActivity => {
                    let index = options
                        .iter()
                        .position(|t| t.activity_id() == self.heatmap_activity_type.id)
                        .unwrap();
                    let index = if matches!(event, Some(DailaEvent::NextHeatmapActivity)) {
                        (index + 1) % options.len()
                    } else {
                        (index + options.len() - 1) % options.len()
                    };
                    self.heatmap_activity_type = self
                        .activity_types
                        .activity_types()
                        .into_iter()
                        .find(|t| t.id == options[index].activity_id())
                        .unwrap()
                        .clone()
                }
            }
        }

        Ok(())
    }
}
