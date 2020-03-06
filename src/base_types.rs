use pancurses::*;
use std::rc::Rc;
use std::cell::RefCell;

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

/// Context passed around to components to effect further actions within the UI
pub struct EventQueue {
    event_queue: Vec<Event>,
}

impl EventQueue {
    pub fn new() -> EventQueue {
        EventQueue {
            event_queue: vec![],
        }
    }

    /// Adds an event to the end of the queue
    pub fn dispatch_event(&mut self, event: EventDetail) {
        self.event_queue.push(Event {
            detail: event,
            handled: false,
        });
    }

    /// Removes a single event from the queue. Returns None if empty.
    pub fn pop(&mut self) -> Option<Event> {
        self.event_queue.pop()
    }
}

/// Represents a component that may live within the UI.
pub trait Component {
    /// Called for all controls in scope for every event that occurs.
    fn on_event(&mut self, event: &mut Event, event_queue: &mut EventQueue);
    /// Called when focus is given by the form.
    /// Return true of the component actually takes focus.
    fn on_gained_focus(&mut self) -> bool;
    /// Called when focus is removed from the control by the form.
    fn on_lost_focus(&mut self);
    /// Called to give the control an opportunity to draw itself
    fn draw(&mut self, window: &Window);
}

/// Since interconnectedness of components can make it tough (or impossible) to
/// build them up on the stack, Form primarily deals with them behind reference
/// counted containers.
pub type ComponentRef = Rc<RefCell<dyn Component>>;

/// Creates a new component ref from a component
pub fn new_component_ref<T>(component: T) -> Rc<RefCell<T>> where T: Component {
    Rc::new(RefCell::<T>::new(component))
}
