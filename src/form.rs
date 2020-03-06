use pancurses::*;
use super::base_types::*;

/// Manages a set of controls.
pub struct Form {
    window: Window,
    components: Vec<ComponentRef>,
    event_queue: EventQueue,
    focus_index: Option<usize>,
}

impl Form {
    /// Creates the form and initializes the pancurses window
    pub fn new() -> Form {
        let window = initscr();
        window.keypad(true); // Set keypad mode
        mousemask(ALL_MOUSE_EVENTS, std::ptr::null_mut()); // Listen to all mouse events
        noecho();
        curs_set(0);

        Form::configure_colors();

        Form {
            window: window,
            components: vec![],
            event_queue: EventQueue::new(),
            focus_index: None,
        }
    }

    /// This may not be great long-term, but should work for now.
    /// This generates a color palette where the eight base colors pair with
    /// all foreground/background combos.
    fn configure_colors() {
        if !has_colors() {
            panic!("This terminal is not supported (no color).");
        }

        start_color();
        if COLOR_PAIRS() < 8 * 8 {
            panic!("This terminal is not supported (not enough color pairs).");
        }

        // Colors run from range 0 (black) to 7 (white). Assign index 0-63 as
        // all possible combinations.
        for fg in 0..8 {
            for bg in 0..8 {
                init_pair(Form::color_index(fg, bg), fg, bg);
            }
        }
    }

    /// Given a foreground and background color, get the color pair index
    pub fn color_index(foreground: i16, background: i16) -> i16 {
        let mut index: i16 = foreground << 3;
        index |= background;
        index
    }

    /// Adds a component to the stack.
    /// Note: Tab order is deduced from add order.
    pub fn push_component(&mut self, component: ComponentRef) {
        self.components.push(component);
    }

    /// Runs the main event loop. Blocks until a FormEvents::Exit event is
    /// processed.
    pub fn run_event_loop(&mut self) {
        self.advance_focus();

        self.window.refresh();

        let mut quit = false;
        while !quit {
            self.window.clear();

            for component in self.components.iter_mut() {
                component.borrow_mut().draw(&self.window);
            }

            self.window.refresh();

            // Handle input first
            let input = self.window.getch();
            if let Some(value) = input {
                let mut event = Event {
                    detail: EventDetail::InputEvent(value),
                    handled: false,
                };

                for component in self.components.iter_mut() {
                    component.borrow_mut().on_event(&mut event, &mut self.event_queue);
                }
            }

            // Now run through the whole immediate queue
            loop {
                match self.event_queue.pop() {
                    Some(mut event) => {
                        for component in self.components.iter_mut() {
                            component.borrow_mut().on_event(&mut event, &mut self.event_queue);
                        }

                        match event.detail {
                            EventDetail::ActionEvent(FormAction::Exit) => quit = true,
                            EventDetail::ActionEvent(FormAction::AdvanceFocus) => self.advance_focus(),
                            _ => { },
                        }
                    }
                    _ => break
                }
            }
        }

        endwin();
    }

    fn advance_focus(&mut self) {
        let start_at = if let Some(index) = self.focus_index {
            self.components[index].borrow_mut().on_lost_focus();
            index + 1
        } else {
            0
        };

        for i in start_at..self.components.len() {
            if self.components[i].borrow_mut().on_gained_focus() {
                self.focus_index = Some(i);
                return;
            }
        }

        for i in 0..start_at {
            if self.components[i].borrow_mut().on_gained_focus() {
                self.focus_index = Some(i);
                return;
            }
        }

        self.focus_index = None;
    }
}
