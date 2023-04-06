use std::io;

use chrono::NaiveDate;
use crossterm::event::{self, Event, KeyCode};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::text::Text;
use tui::widgets::{Block, Borders, Paragraph};
use tui::Terminal;

use crate::activites::{
    self, ActivitiesStore, Activity, ActivityOption, ActivityType, ActivityTypesStore,
};
use crate::activity_selector::{ActivitySelector, ActivitySelectorValue};
use crate::file::File;
use crate::heatmap::HeatMap;

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
        let activity_types = ActivityTypesStore::load();
        let first_type = activity_types.activity_types()[0].clone();

        Self {
            activity_types: activity_types,
            activities: ActivitiesStore::load(),
            active_date: chrono::Local::now().date_naive(),
            heatmap_activity_type: first_type,
        }
    }

    pub fn instructions_block(&self) -> Paragraph {
        let instructions = vec![
            ("j", "previous day"),
            ("k", "next day"),
            ("t", "today"),
            ("n", "display previous activity on heatmap"),
            ("m", "display next activity on heatmap"),
            ("%d", "toggle activity"),
            ("q", "quit"),
        ];
        let strings: Vec<String> = instructions
            .into_iter()
            .map(|(c, message)| format!("{c}: {message}"))
            .collect();
        let string = strings.join("\n");

        Paragraph::new(Text::raw(string.to_owned())).block(Block::default().borders(Borders::ALL))
    }

    pub fn run_daila<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), io::Error> {
        let mut running = true;

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
                    .split(Rect {
                        x: frame_size.x,
                        y: frame_size.y,
                        width: heatmap.width(),
                        height: frame_size.height,
                    });

                frame.render_widget(selector, chunks[0]);
                frame.render_widget(heatmap, chunks[1]);
                frame.render_widget(self.instructions_block(), chunks[2]);
            })?;
            if let Ok(Event::Key(key)) = event::read() {
                match key.code {
                    // Handle quit.
                    KeyCode::Char('q') => running = false,
                    // Handle activity selection.
                    KeyCode::Char(c) if c.is_digit(10) => {
                        let index = c.to_digit(10).unwrap() as usize;
                        // Toggle the activity.
                        if let Some(option) = options.get(index - 1) {
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
                    // Change the current date.
                    KeyCode::Char('j') => self.active_date = self.active_date.pred_opt().unwrap(),
                    KeyCode::Char('k') => self.active_date = self.active_date.succ_opt().unwrap(),
                    KeyCode::Char('t') => self.active_date = chrono::Local::now().date_naive(),
                    // Change the activity type displayed in the heatmap.
                    // TODO: This is hacky and I don't like it. Rewrite this to be better.
                    //       (this includes the heatmap_activity_type field!)
                    // Edit: This _was_ hacky. This block as descended below that status. No. Just no. No.
                    KeyCode::Char(c) if c == 'n' || c == 'm' => {
                        let index = options
                            .iter()
                            .position(|t| t.activity_id() == self.heatmap_activity_type.id)
                            .unwrap();
                        let index = if c == 'm' {
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
                    // ---safety barrier---
                    _ => {}
                }
                if let KeyCode::Char('q') = key.code {
                    running = false;
                }
            }
        }

        // Save any unsaved changes.
        self.activity_types.save();
        self.activities.save();

        Ok(())
    }
}
