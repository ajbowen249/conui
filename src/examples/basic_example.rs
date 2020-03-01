extern crate conui;

use async_trait::async_trait;
use conui::*;
use pancurses::{
    ALL_MOUSE_EVENTS,
    endwin,
    getmouse,
    initscr,
    init_pair,
    Input,
    mousemask,
    Window,
    COLOR_BLUE,
    COLOR_RED,
};

struct Button {
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
        init_pair(0, COLOR_BLUE, COLOR_RED);
        window.color_set(0);
        window.mvprintw(4, 4, format!("Quit"));
    }
}

#[tokio::main]
async fn main() {
    let mut form = Form::new();
    let btn = Button::new();

    form.push_component(Box::new(btn));

    form.run_event_loop().await;
}
