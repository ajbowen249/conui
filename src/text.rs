use pancurses::*;
use super::base_types::*;
use super::utils::TextBinder;
use super::form::Form;
use std::rc::Rc;
use std::cell::RefCell;

/// A block of text in the UI
/// When building, use TextBuilder::set_text to set immutable text.
/// For dynamic text action, set a TextBinder via TextBuilder::set_text_binder,
/// which will allow text to be mutated later on.
pub struct Text {
    text: String,
    text_binder: Option<TextBinder>,
    x_pos: i32,
    y_pos: i32,
    fg_color: i16,
    bg_color: i16,
}

impl Component for Text {
    // Text does not care about events or focus. It's just somet text.
    fn on_event(&mut self, _: &mut Event, _: &mut Vec<Event>) { }
    fn on_gained_focus(&mut self) -> bool { false }
    fn on_lost_focus(&mut self) { }

    fn draw(&mut self, window: &Window) {
        window.color_set(Form::color_index(self.fg_color, self.bg_color));
        window.mvprintw(self.y_pos, self.x_pos, format!("{}", self.get_actual_text()));
    }
}

impl Text {
    fn get_actual_text(&self) -> String {
        match &self.text_binder {
            Some(binder) => binder.get(),
            None => self.text.clone(),
        }
    }
}

/// Builder for a Text component
pub struct TextBuilder {
    text: String,
    text_binder: Option<TextBinder>,
    x_pos: i32,
    y_pos: i32,
    fg_color: i16,
    bg_color: i16,
}

impl TextBuilder {
    /// Creates a new TextBuilder with default options
    pub fn new() -> TextBuilder {
        TextBuilder {
            text: String::new(),
            text_binder: None,
            x_pos: 0,
            y_pos: 0,
            fg_color: COLOR_WHITE,
            bg_color: COLOR_BLACK,
        }
    }

    /// Set the displayed text
    pub fn set_text(mut self, text: &str) -> TextBuilder {
        self.text = String::from(text);
        self
    }

    /// Allow text to be set via a TextBinder
    /// Note: Setting this will override anything set by set_text
    pub fn set_text_binder(mut self, binder: TextBinder) -> TextBuilder {
        self.text_binder = Some(binder);
        self
    }

    /// Set placement on the screen
    pub fn set_position(mut self, x_pos: i32, y_pos: i32) -> TextBuilder {
        self.x_pos = x_pos;
        self.y_pos = y_pos;
        self
    }

    /// Set the color of the text itself
    pub fn set_fg_color(mut self, fg_color: i16) -> TextBuilder {
        self.fg_color = fg_color;
        self
    }

    /// Set the background color
    pub fn set_bg_color(mut self, bg_color: i16) -> TextBuilder {
        self.bg_color = bg_color;
        self
    }

    /// Builds the Text component with assigned options
    pub fn build(self) -> Rc<RefCell<Text>> {
        new_component_ref(Text {
            text: self.text,
            text_binder: self.text_binder,
            x_pos: self.x_pos,
            y_pos: self.y_pos,
            fg_color: self.fg_color,
            bg_color: self.bg_color,
        })
    }
}
