use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use daila::Daila;
use std::io;

use ratatui::{backend::CrosstermBackend, Terminal};

mod activites;
mod activity_popup;
mod activity_selector;
mod daila;
mod file;
mod heatmap;

fn main() -> Result<(), io::Error> {
    // Setup.
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run application.
    Daila::new().run_daila(&mut terminal)?;

    // Cleanup.
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
