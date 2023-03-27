use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders},
    Frame, Terminal,
};

mod heatmap;

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
    loop {
        terminal.draw(draw_daila)?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                break;
            }
        }
    }

    Ok(())
}

fn draw_daila<B: Backend>(frame: &mut Frame<B>) {
    let size = frame.size();
    let block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::Green));
    // .border_style(Style::default().fg(Color::White).bg(Color::Green));
    frame.render_widget(block, Rect::new(0, 0, 1, 1));
}

fn draw_activity_map<B: Backend>(frame: &mut Frame<B>) {}
