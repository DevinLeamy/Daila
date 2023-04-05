use activity_selector::{ActivitySelector, ActivitySelectorValue};
use chrono::Days;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use daila::Daila;
use file::File;
use heatmap::{CalendarDate, HeatMap};
use rand::{thread_rng, Rng};
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    Frame, Terminal,
};

mod activites;
mod activity_selector;
mod daila;
mod file;
mod heatmap;

use activites::{ActivitiesStore, Activity, ActivityId, ActivityOption, ActivityTypesStore};

fn main() -> Result<(), io::Error> {
    // Setup.
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run application.
    run_daila(&mut terminal)?;

    // Cleanup.
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

fn run_daila<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), io::Error> {
    let (mut activity_types, mut activities) = Daila::init();

    // let mut rand = thread_rng();

    // Generate random data.
    // let current_day = CalendarDate::from_ymd_opt(2022, 1, 1).unwrap();
    // for i in 0..1000 {
    //     let date = current_day + Days::new(i);

    //     if rand.gen::<f32>() > 0.2 {
    //         activities.add_activity(Activity::new(activity_types.activity_types()[0].id, date));
    //     }
    // }

    let mut running = true;
    let active_date = chrono::Local::now().date_naive();

    while running {
        let options =
            activites::activity_options(&activity_types, &activities, active_date.clone());
        terminal.draw(|frame| {
            let frame_size = frame.size();
            let heatmap = HeatMap::default().values(activities.activities());
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
                    if let Some(option) = options.get(index) {
                        let activity = Activity::new(option.activity_id(), active_date.clone());
                        if option.completed() {
                            // Toggle off.
                            activities.remove_activity(activity);
                        } else {
                            // Toggle on.
                            activities.add_activity(activity);
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
    activity_types.save();
    activities.save();

    Ok(())
}
