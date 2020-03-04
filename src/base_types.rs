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

/// Represents a component that may live within the UI.
pub trait Component {
    /// Called for all controls in scope for every event that occurs.
    fn on_event(&mut self, event: &mut Event, event_queue: &mut Vec<Event>);
    /// Called when focus is given by the form.
    /// Return true of the component actually takes focus.
    fn on_gained_focus(&mut self) -> bool;
    /// Called when focus is removed from the control by the form.
    fn on_lost_focus(&mut self);
    /// Called to give the control an opportunity to draw itself
    fn draw(&mut self, window: &Window);
}

/// Not meant to be used. Is here because the compiler wants at least one
/// implementation of Component for dyn to work.
pub struct ComponentStub { }
impl Component for ComponentStub {
    fn on_event(&mut self, _: &mut Event, _: &mut Vec<Event>) { }
    fn on_gained_focus(&mut self) -> bool { false }
    fn on_lost_focus(&mut self) { }
    fn draw(&mut self, _: &Window) { }
}

/// Since interconnectedness of components can make it tough (or impossible) to
/// build them up on the stack, Form primarily deals with them behind reference
/// counted containers.
pub type ComponentRef = Rc<RefCell<dyn Component>>;

/// Creates a new component ref from a component (use with result of builder)
pub fn new_component_ref<T>(component: T) -> Rc<RefCell<T>> where T: Component {
    Rc::new(RefCell::<T>::new(component))
}
