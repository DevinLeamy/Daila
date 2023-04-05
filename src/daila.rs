use std::io;

use crossterm::event::{self, Event, KeyCode};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::Terminal;

use crate::activites::{self, ActivitiesStore, Activity, ActivityOption, ActivityTypesStore};
use crate::activity_selector::{ActivitySelector, ActivitySelectorValue};
use crate::file::File;
use crate::heatmap::HeatMap;

pub struct Daila {
    activity_types: ActivityTypesStore,
    activities: ActivitiesStore,
}

impl Daila {
    pub fn new() -> Self {
        Self {
            activity_types: ActivityTypesStore::load(),
            activities: ActivitiesStore::load(),
        }
    }

    pub fn run_daila<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), io::Error> {
        let mut running = true;
        let active_date = chrono::Local::now().date_naive();

        while running {
            let options = activites::activity_options(
                &self.activity_types,
                &self.activities,
                active_date.clone(),
            );
            terminal.draw(|frame| {
                let frame_size = frame.size();
                let heatmap = HeatMap::default().values(
                    self.activities
                        .activities_with_type(self.activity_types.activity_types()[5]),
                );
                let selector = ActivitySelector::<ActivityOption>::default()
                    .values(options.iter().map(|o| o).collect());

                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(Rect {
                        x: frame_size.x,
                        y: frame_size.y,
                        width: heatmap.width(),
                        height: frame_size.height,
                    });

                frame.render_widget(selector, chunks[0]);
                frame.render_widget(heatmap, chunks[1]);
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
                            let activity = Activity::new(option.activity_id(), active_date.clone());
                            if option.completed() {
                                // Toggle off.
                                self.activities.remove_activity(activity);
                            } else {
                                // Toggle on.
                                self.activities.add_activity(activity);
                            }
                        } else {
                            println!("Invalid activity index: {}", index);
                        }
                    }
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
