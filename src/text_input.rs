use pancurses::*;
use super::base_types::*;
use super::form::Form;
use super::utils::*;
use std::iter::FromIterator;
use std::rc::Rc;
use std::cell::RefCell;

/// An Input within the UI
pub struct TextInput {
    has_focus: bool,
    label: String,
    text_binder: TextBinder,
    x_pos: i32,
    y_pos: i32,
    box_width: u32,
    neutral_fg_color: i16,
    focus_fg_color: i16,
    neutral_bg_color: i16,
    focus_bg_color: i16,
}

impl Component for TextInput {
    fn on_event(&mut self, event: &mut Event, event_queue: &mut EventQueue) {
        if event.handled {
            return;
        }

        match event.detail {
            EventDetail::InputEvent(input_event) => {
                match input_event {
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
                                // TODO: Request focus
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
                    Input::Character(ch) => {
                        if !self.has_focus {
                            return;
                        }
                        event.handled = true;
                        let mut text = self.text_binder.get();
                        text.push(ch);
                        self.text_binder.set(text.as_str());
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

impl TextInput {
    fn get_width(&self) -> u32 {
        self.box_width + 2
    }

    fn get_height(&self) -> u32 {
        3
    }

    fn get_text(&self) -> String {
        self.text_binder.get()
    }
}

/// Builder for an input component
pub struct TextInputBuilder {
    label: String,
    text_binder: TextBinder,
    x_pos: i32,
    y_pos: i32,
    box_width: u32,
    neutral_fg_color: i16,
    focus_fg_color: i16,
    neutral_bg_color: i16,
    focus_bg_color: i16,
}

impl TextInputBuilder {
    /// Create a new TextInputBuilder with default data
    pub fn new() -> TextInputBuilder {
        TextInputBuilder {
            label: String::new(),
            text_binder: TextBinder::new(""),
            x_pos: 0,
            y_pos: 0,
            box_width: 0,
            neutral_fg_color: COLOR_WHITE,
            focus_fg_color: COLOR_WHITE,
            neutral_bg_color: COLOR_BLUE,
            focus_bg_color: COLOR_RED,
        }
    }

    /// Allow text to be read via a TextBinder
    pub fn set_text_binder(mut self, binder: TextBinder) -> TextInputBuilder {
        self.text_binder = binder;
        self
    }

    /// Set the TextInput's placement within the UI
    pub fn set_position(mut self, x_pos: i32, y_pos: i32) -> TextInputBuilder {
        self.x_pos = x_pos;
        self.y_pos = y_pos;
        self
    }

    pub fn set_box_width(mut self, box_width: u32) -> TextInputBuilder {
        self.box_width = box_width;
        self
    }

    /// Set the color of the text and border when not in focus
    pub fn set_neutral_fg_color(mut self, neutral_fg_color: i16) -> TextInputBuilder {
        self.neutral_fg_color = neutral_fg_color;
        self
    }

    /// Set the color of the text and border when focused
    pub fn set_focus_fg_color(mut self, focus_fg_color: i16) -> TextInputBuilder {
        self.focus_fg_color = focus_fg_color;
        self
    }

    /// Set the background color when not in focus
    pub fn set_neutral_bg_color(mut self, neutral_bg_color: i16) -> TextInputBuilder {
        self.neutral_bg_color = neutral_bg_color;
        self
    }

    /// Set the background color when focused
    pub fn set_focus_bg_color(mut self, focus_bg_color: i16) -> TextInputBuilder {
        self.focus_bg_color = focus_bg_color;
        self
    }

    /// Builds the TextInput component with set options
    pub fn build(self) -> Rc<RefCell<TextInput>> {
        new_component_ref(TextInput {
            has_focus: false,
            label: self.label,
            text_binder: self.text_binder,
            x_pos: self.x_pos,
            y_pos: self.y_pos,
            box_width: self.box_width,
            neutral_fg_color: self.neutral_fg_color,
            focus_fg_color: self.focus_fg_color,
            neutral_bg_color: self.neutral_bg_color,
            focus_bg_color: self.focus_bg_color,
        })
    }
}
