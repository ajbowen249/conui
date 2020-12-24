use pancurses::*;
use super::base_types::*;
use super::form::Form;
use super::utils::*;
use std::iter::FromIterator;
use std::rc::Rc;
use std::cell::RefCell;

/// A Button within the UI
pub struct Button<F> {
    name: String,
    has_focus: bool,
    action: Option<F>,
    label: String,
    x_pos: i32,
    y_pos: i32,
    neutral_fg_color: i16,
    focus_fg_color: i16,
    neutral_bg_color: i16,
    focus_bg_color: i16,
}

impl<F> Component for Button<F> where F: FnMut(&mut EventQueue) {
    fn get_name(&self) -> &String { &self.name }

    fn on_event(&mut self, event: &mut Event, event_queue: &mut EventQueue) {
        if event.handled {
            return;
        }

        match event.detail {
            EventDetail::InputEvent(input_event) => {
                match input_event {
                    Input::Character('\n') => {
                        if !self.has_focus {
                            return;
                        }
                        event.handled = true;
                        self.maybe_execute_action(event_queue);
                    },
                    Input::KeyMouse => {
                        if let Ok(mouse_event) = getmouse() {
                            if is_in_window(
                                self.x_pos,
                                self.y_pos,
                                self.get_width(),
                                self.get_height(),
                                mouse_event.x,
                                mouse_event.y
                            ) {
                                // Focus does not matter since it was a direct click.
                                event.handled = true;
                                self.maybe_execute_action(event_queue);
                            }
                        }
                    },
                    Input::Character('\t') => {
                        if !self.has_focus {
                            return;
                        }
                        event.handled = true;
                        event_queue.dispatch_event(EventDetail::ActionEvent(FormAction::AdvanceFocus));
                    },
                    _ => { },
                }
            },
            _ => { },
        }
    }

    fn on_gained_focus(&mut self) -> bool {
        self.has_focus = true;
        true
    }

    fn on_lost_focus(&mut self) {
        self.has_focus = false;
    }

    fn draw(&mut self, window: &Window) {
        let fg_color = if self.has_focus { self.focus_fg_color } else { self.neutral_fg_color };
        let bg_color = if self.has_focus { self.focus_bg_color } else { self.neutral_bg_color };
        window.color_set(Form::color_index(fg_color, bg_color));

        let horizontal_border = String::from_iter(vec!['─'; self.label.chars().count()]);
        window.mvprintw(self.y_pos,     self.x_pos, format!("┌{}┐", horizontal_border));
        window.mvprintw(self.y_pos + 1, self.x_pos, format!("│{}│", self.label));
        window.mvprintw(self.y_pos + 2, self.x_pos, format!("└{}┘", horizontal_border));
    }
}

impl<F> Button<F> where F: FnMut(&mut EventQueue) {
    fn maybe_execute_action(&mut self, event_queue: &mut EventQueue) {
        if let Some(action) = &mut self.action {
            action(event_queue);
        }
    }

    fn get_width(&self) -> u32 {
        self.label.chars().count() as u32 + 2
    }

    fn get_height(&self) -> u32 {
        3
    }
}

/// Builder for a Button component
pub struct ButtonBuilder<F> {
    name: String,
    action: Option<F>,
    label: String,
    x_pos: i32,
    y_pos: i32,
    neutral_fg_color: i16,
    focus_fg_color: i16,
    neutral_bg_color: i16,
    focus_bg_color: i16,
}

impl<F> ButtonBuilder<F> where F: FnMut(&mut EventQueue) {
    /// Create a new ButtonBuilder with default data
    pub fn new(name: &String) -> ButtonBuilder<F> {
        ButtonBuilder {
            name: name.to_string(),
            action: None,
            label: String::new(),
            x_pos: 0,
            y_pos: 0,
            neutral_fg_color: COLOR_WHITE,
            focus_fg_color: COLOR_WHITE,
            neutral_bg_color: COLOR_BLUE,
            focus_bg_color: COLOR_RED,
        }
    }

    /// Assign a callback to run when the button is activated
    pub fn set_action(mut self, f: F) -> ButtonBuilder<F> {
        self.action = Some(f);
        self
    }

    /// Set the text within the button
    pub fn set_label(mut self, label: &str) -> ButtonBuilder<F> {
        self.label = String::from(label);
        self
    }

    /// Set the button's placement within the UI
    pub fn set_position(mut self, x_pos: i32, y_pos: i32) -> ButtonBuilder<F> {
        self.x_pos = x_pos;
        self.y_pos = y_pos;
        self
    }

    /// Set the color of the text and border when not in focus
    pub fn set_neutral_fg_color(mut self, neutral_fg_color: i16) -> ButtonBuilder<F> {
        self.neutral_fg_color = neutral_fg_color;
        self
    }

    /// Set the color of the text and border when focused
    pub fn set_focus_fg_color(mut self, focus_fg_color: i16) -> ButtonBuilder<F> {
        self.focus_fg_color = focus_fg_color;
        self
    }

    /// Set the background color when not in focus
    pub fn set_neutral_bg_color(mut self, neutral_bg_color: i16) -> ButtonBuilder<F> {
        self.neutral_bg_color = neutral_bg_color;
        self
    }

    /// Set the background color when focused
    pub fn set_focus_bg_color(mut self, focus_bg_color: i16) -> ButtonBuilder<F> {
        self.focus_bg_color = focus_bg_color;
        self
    }

    /// Builds the button component with set options
    pub fn build(self) -> Rc<RefCell<Button<F>>> {
        new_component_ref(Button {
            has_focus: false,
            name: self.name,
            label: self.label,
            action: self.action,
            x_pos: self.x_pos,
            y_pos: self.y_pos,
            neutral_fg_color: self.neutral_fg_color,
            focus_fg_color: self.focus_fg_color,
            neutral_bg_color: self.neutral_bg_color,
            focus_bg_color: self.focus_bg_color,
        })
    }
}
