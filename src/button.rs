use pancurses::*;
use super::base_types::*;
use super::form::Form;
use std::iter::FromIterator;

pub struct Button<F> {
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

impl<F> Component for Button<F> where F: FnMut(&mut Vec<Event>) {
    fn on_event(&mut self, event: &mut Event, event_queue: &mut Vec<Event>) {
        if event.handled || !self.has_focus {
            return;
        }

        match event.detail {
            EventDetail::InputEvent(input_event) => {
                match input_event {
                    Input::Character('\n') => {
                        event.handled = true;
                        if let Some(action) = &mut self.action {
                            action(event_queue);
                        }
                    },
                    Input::Character('\t') => {
                        event.handled = true;
                        event_queue.push(Event {
                            detail: EventDetail::ActionEvent(FormAction::AdvanceFocus),
                            handled: false
                        });
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

pub struct ButtonBuilder<F> {
    action: Option<F>,
    label: String,
    x_pos: i32,
    y_pos: i32,
    neutral_fg_color: i16,
    focus_fg_color: i16,
    neutral_bg_color: i16,
    focus_bg_color: i16,
}

impl<F> ButtonBuilder<F> where F: FnMut(&mut Vec<Event>) {
    pub fn new() -> ButtonBuilder<F> {
        ButtonBuilder {
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

    pub fn set_action(mut self, f: F) -> ButtonBuilder<F> {
        self.action = Some(f);
        self
    }

    pub fn set_label(mut self, label: &str) -> ButtonBuilder<F> {
        self.label = String::from(label);
        self
    }

    pub fn set_position(mut self, x_pos: i32, y_pos: i32) -> ButtonBuilder<F> {
        self.x_pos = x_pos;
        self.y_pos = y_pos;
        self
    }

    pub fn set_neutral_fg_color(mut self, neutral_fg_color: i16) -> ButtonBuilder<F> {
        self.neutral_fg_color = neutral_fg_color;
        self
    }

    pub fn set_focus_fg_color(mut self, focus_fg_color: i16) -> ButtonBuilder<F> {
        self.focus_fg_color = focus_fg_color;
        self
    }

    pub fn set_neutral_bg_color(mut self, neutral_bg_color: i16) -> ButtonBuilder<F> {
        self.neutral_bg_color = neutral_bg_color;
        self
    }

    pub fn set_focus_bg_color(mut self, focus_bg_color: i16) -> ButtonBuilder<F> {
        self.focus_bg_color = focus_bg_color;
        self
    }

    pub fn build(self) -> Button<F> {
        Button {
            has_focus: false,
            label: self.label,
            action: self.action,
            x_pos: self.x_pos,
            y_pos: self.y_pos,
            neutral_fg_color: self.neutral_fg_color,
            focus_fg_color: self.focus_fg_color,
            neutral_bg_color: self.neutral_bg_color,
            focus_bg_color: self.focus_bg_color,
        }
    }
}
