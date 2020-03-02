use async_trait::async_trait;
use pancurses::*;
use super::base_types::*;

pub struct Button<F> {
    has_focus: bool,
    action: Option<F>,
    label: String,
    x_pos: i32,
    y_pos: i32,
}

#[async_trait]
impl<F> Component for Button<F> where F: Fn(&mut Vec<Event>) + Send {
    async fn on_event(&mut self, event: &mut Event, event_queue: &mut Vec<Event>) {
        if event.handled || !self.has_focus {
            return;
        }

        match event.detail {
            EventDetail::InputEvent(input_event) => {
                match input_event {
                    Input::Character('\n') => {
                        event.handled = true;
                        if let Some(action) = &self.action {
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

    async fn on_gained_focus(&mut self) -> bool {
        self.has_focus = true;
        true
    }

    async fn on_lost_focus(&mut self) {
        self.has_focus = false;
    }

    fn draw(&mut self, window: &Window) {
        window.color_set(if self.has_focus { 1 } else { 2 });
        window.mvprintw(self.y_pos, self.x_pos, format!("{}", self.label));
    }
}

pub struct ButtonBuilder<F> {
    action: Option<F>,
    label: String,
    x_pos: i32,
    y_pos: i32,
}

impl<F> ButtonBuilder<F> where F: Fn(&mut Vec<Event>) + Send {
    pub fn new() -> ButtonBuilder<F> {
        ButtonBuilder {
            action: None,
            label: String::new(),
            x_pos: 0,
            y_pos: 0,
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

    pub fn build(self) -> Button<F> {
        Button {
            has_focus: false,
            label: self.label,
            action: self.action,
            x_pos: self.x_pos,
            y_pos: self.y_pos,
        }
    }
}
