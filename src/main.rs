use chrono::Days;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use heatmap::{CalendarDate, HeatMap};
use rand::{thread_rng, Rng};
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    Frame, Terminal,
};

mod activites;
mod heatmap;

use activites::{ActivitiesStore, Activity, ActivityTypesStore};

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
    let mut activity_types = ActivityTypesStore::new();
    let mut activites = ActivitiesStore::new();

    let running_id = activity_types.create_new_activity(String::from("Running"));
    let mut rand = thread_rng();

    activites.add_activity(Activity::new(
        running_id,
        CalendarDate::from_ymd_opt(2020, 1, 1).unwrap(),
    ));

    // Generate random data.
    let current_day = CalendarDate::from_ymd_opt(2022, 1, 1).unwrap();
    for i in 0..1000 {
        let date = current_day + Days::new(i);

        if rand.gen::<f32>() > 0.2 {
            activites.add_activity(Activity::new(running_id, date));
        }
    }

    loop {
        let activites_clone = activites.clone();
        terminal.draw(move |frame| {
            draw_daila(frame, activites_clone);
        })?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                break;
            }
        }
    }

    Ok(())
}

fn draw_daila<B: Backend>(frame: &mut Frame<B>, activites_store: ActivitiesStore) {
    draw_activity_map(frame, &activites_store)
}

fn draw_activity_map<B: Backend>(frame: &mut Frame<B>, activites_store: &ActivitiesStore) {
    let heatmap = HeatMap::default().values(activites_store.activities());
    frame.render_widget(heatmap, frame.size());
}
