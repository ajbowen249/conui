use pancurses::*;
use super::base_types::*;

/// Manages a set of controls.
pub struct Form {
    components: Vec<Box<dyn Component>>,
    event_queue: Vec<Event>,
    focus_index: Option<usize>,
}

impl Form {
    pub fn new() -> Form {
        Form {
            components: vec![],
            event_queue: vec![],
            focus_index: None,
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
        noecho();

        let mut bg = COLOR_BLACK;
        start_color();
        if use_default_colors() == OK {
            bg = -1;
        }

        init_pair(1, COLOR_BLUE, bg);
        init_pair(2, COLOR_WHITE, bg);

        self.advance_focus().await;

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
                            EventDetail::ActionEvent(FormAction::AdvanceFocus) => self.advance_focus().await,
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

    async fn advance_focus(&mut self) {
        let start_at = if let Some(index) = self.focus_index {
            self.components[index].on_lost_focus().await;
            index + 1
        } else {
            0
        };

        for i in start_at..self.components.len() {
            if self.components[i].on_gained_focus().await {
                self.focus_index = Some(i);
                return;
            }
        }

        for i in 0..start_at {
            if self.components[i].on_gained_focus().await {
                self.focus_index = Some(i);
                return;
            }
        }

        self.focus_index = None;
    }
}
