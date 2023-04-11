use crossterm::event::Event;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Clear, StatefulWidget},
    Frame,
};

pub trait Popup<S>: StatefulWidget<State = S> {
    // Type of actions that the popup must handle.
    type Action;

    /**
     * Handles an input event and update internal state.
     */
    fn handle_event(event: &Event, state: &mut S) -> Option<Self::Action>;
}

pub fn render_in_frame<B: Backend, S, P: Popup<S>>(
    frame: &mut Frame<B>,
    area: &Rect,
    width_percentage: u16,
    height_percentage: u16,
    popup: P,
    state: &mut S,
) {
    let area = centered_frame(area, width_percentage, height_percentage);
    frame.render_widget(Clear, area);
    frame.render_stateful_widget(popup, area, state);
}

/**
 * Create the frame for a centered popup.
 */
fn centered_frame(area: &Rect, width_percentage: u16, height_percentage: u16) -> Rect {
    // Center vertically.
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
        .split(area.to_owned());

    // Center horizontally.
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - width_percentage) / 2),
                Constraint::Percentage(width_percentage),
                Constraint::Percentage((100 - width_percentage) / 2),
            ]
            .as_ref(),
        )
        .split(popup_area[1])[1]
}
