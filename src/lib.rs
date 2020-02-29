use async_trait::async_trait;
use pancurses::{Input};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

/// Non input-related events
#[derive(Copy, Clone)]
pub enum FormEvent {
    AdvanceFocus,
    Exit,
}

/// Represents events that occur within the UI.
pub union EventDetail {
    input: Input,
    form_event: FormEvent,
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
    async fn on_event(&mut self, event: &mut Event);
    /// Called when focus is given by the form.
    /// Return true of the component actually takes focus.
    async fn on_gained_focus(&mut self) -> bool;
    /// Called when focus is removed from the control by the form.
    async fn on_lost_focus(&mut self);
}

/// Not meant to be used. Is here because the compiler wants at least one
/// implementation of Component for dyn to work.
struct ComponentStub { }
#[async_trait]
impl Component for ComponentStub {
    async fn on_event(&mut self, _: &mut Event) { }
    async fn on_gained_focus(&mut self) -> bool { false }
    async fn on_lost_focus(&mut self) { }
}

/// Allows a control to push custom events to the queue.
pub trait EventQueue {
    fn push_event(&mut self, event: Event);
}

/// Manages a set of controls.
pub struct Form {
    components: Vec<Box<dyn Component>>,
}

impl Form {
    /// Adds a component to the stack.
    /// Note: Tab order is deduced from add order.
    pub fn push_component(&mut self, component: Box<dyn Component>) {
        self.components.push(component);
    }

    /// Runs the main event loop. Blocks until a FormEvents::Exit event is
    /// processed.
    pub async fn run_event_loop(&mut self) {

    }
}

impl EventQueue for Form {
    fn push_event(&mut self, event: Event) { }
}
