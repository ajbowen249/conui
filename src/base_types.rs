use async_trait::async_trait;
use pancurses::*;

/// Non input-related events
#[derive(Copy, Clone)]
pub enum FormAction {
    Init,
    AdvanceFocus,
    Exit,
}

/// Represents events that occur within the UI.
pub enum EventDetail {
    InputEvent(Input),
    ActionEvent(FormAction),
}

/// Represents an event in progress.
pub struct Event {
    /// The event the occurred
    pub detail: EventDetail,
    /// Whether a control has "handled" the event.
    /// The event will always be passed to all controls. This flag is meant to
    /// signal to other controls, not to stop propogation.
    pub handled: bool,
}

/// Represents a component that may live within the UI.
#[async_trait]
pub trait Component {
    /// Called for all controls in scope for every event that occurs.
    async fn on_event(&mut self, event: &mut Event, event_queue: &mut Vec<Event>);
    /// Called when focus is given by the form.
    /// Return true of the component actually takes focus.
    async fn on_gained_focus(&mut self) -> bool;
    /// Called when focus is removed from the control by the form.
    async fn on_lost_focus(&mut self);
    /// Called to give the control an opportunity to draw itself
    fn draw(&mut self, window: &Window);
}

/// Not meant to be used. Is here because the compiler wants at least one
/// implementation of Component for dyn to work.
pub struct ComponentStub { }
#[async_trait]
impl Component for ComponentStub {
    async fn on_event(&mut self, _: &mut Event, _: &mut Vec<Event>) { }
    async fn on_gained_focus(&mut self) -> bool { false }
    async fn on_lost_focus(&mut self) { }
    fn draw(&mut self, _: &Window) { }
}
