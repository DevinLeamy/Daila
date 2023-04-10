use crossterm::event::Event;
use ratatui::widgets::StatefulWidget;

pub trait Popup<S>: StatefulWidget<State = S> {
    // Type of actions that the popup must handle.
    type Action;

    fn action_from_event(event: &Event) -> Option<Self::Action>;

    // /**
    //  * Handles an input event using a provided handler.
    //  */
    // fn handle_event(
    //     &mut self,
    //     event: &Event,
    //     mut action_handler: impl FnMut(Option<Self::Action>),
    // ) {
    //     let action = self.action_from_event(event);
    //     action_handler(action);
    // }
}
