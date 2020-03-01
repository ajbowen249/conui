use pancurses::*;
use super::base_types::*;

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
