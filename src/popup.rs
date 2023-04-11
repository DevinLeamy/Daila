use crossterm::event::Event;
use ratatui::widgets::StatefulWidget;

pub trait Popup<S>: StatefulWidget<State = S> {
    // Type of actions that the popup must handle.
    type Action;

    /**
     * Handles an input event and update internal state.
     */
    fn handle_event(event: &Event, state: &mut S) -> Option<Self::Action> {
        None
    }
}
