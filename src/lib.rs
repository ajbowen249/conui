use async_trait::async_trait;
use pancurses::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

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
struct ComponentStub { }
#[async_trait]
impl Component for ComponentStub {
    async fn on_event(&mut self, _: &mut Event, event_queue: &mut Vec<Event>) { }
    async fn on_gained_focus(&mut self) -> bool { false }
    async fn on_lost_focus(&mut self) { }
    fn draw(&mut self, _: &Window) { }
}

/// Manages a set of controls.
pub struct Form {
    components: Vec<Box<dyn Component>>,
    event_queue: Vec<Event>,
}

impl Form {
    pub fn new() -> Form {
        Form {
            components: vec![],
            event_queue: vec![],
        }
    }

    /// Adds a component to the stack.
    /// Note: Tab order is deduced from add order.
    pub fn push_component(&mut self, component: Box<dyn Component>) {
        self.components.push(component);
    }

    /// Runs the main event loop. Blocks until a FormEvents::Exit event is
    /// processed.
    pub async fn run_event_loop(&mut self) {
        let window = initscr();

        window.keypad(true); // Set keypad mode
        mousemask(ALL_MOUSE_EVENTS, std::ptr::null_mut()); // Listen to all mouse events
        let mut bg = COLOR_BLACK;
        start_color();
        if use_default_colors() == OK {
            bg = -1;
        }

        init_pair(1, COLOR_RED, bg);
        init_pair(2, COLOR_GREEN, bg);
    
        window.refresh();

        let mut quit = false;
        while !quit {
            for component in self.components.iter_mut() {
                component.draw(&window);
            }

            window.refresh();

            // Handle input first
            let input = window.getch();
            if let Some(value) = input {
                let mut event = Event {
                    detail: EventDetail::InputEvent(value),
                    handled: false,
                };

                for component in self.components.iter_mut() {
                    component.on_event(&mut event, &mut self.event_queue).await;
                }
            }

            // Now run through the whole immediate queue
            loop {
                match self.event_queue.pop() {
                    Some(mut event) => {
                        for component in self.components.iter_mut() {
                            component.on_event(&mut event, &mut self.event_queue).await;
                        }

                        match event.detail {
                            EventDetail::ActionEvent(FormAction::Exit) => quit = true,
                            _ => { },
                        }
                    }
                    _ => break
                }
            }
        }

        endwin();
    }

    /// Add an event to the queue
    pub fn push_event(&mut self, event: Event) {
        self.event_queue.push(event);
    }
}
