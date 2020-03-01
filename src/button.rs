use async_trait::async_trait;
use pancurses::*;
use super::base_types::*;

pub struct Button {
    has_focus: bool,
}

impl Button {
    pub fn new() -> Button {
        Button {
            has_focus: false,
        }
    }
}

#[async_trait]
impl Component for Button {
    async fn on_event(&mut self, event: &mut Event, event_queue: &mut Vec<Event>) {
        if event.handled {
            return;
        }

        match event.detail {
            EventDetail::InputEvent(input_event) => {
                match input_event {
                    Input::Character('\n') => {
                        event.handled = true;
                        event_queue.push(Event {
                            detail: EventDetail::ActionEvent(FormAction::Exit),
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
        window.color_set(1);
        window.mvprintw(4, 4, format!("Quit"));
        window.color_set(2);
        window.mvprintw(5, 4, format!("Quit"));
    }
}
