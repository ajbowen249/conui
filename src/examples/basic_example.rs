extern crate conui;
use conui::*;
use pancurses::*;

fn main() {
    let mut form = Form::new();

    form.push_component(Box::new(ButtonBuilder::new()
        .set_label("Quit 1")
        .set_position(1, 1)
        .set_action(|q| {
            q.push(Event {
                detail: EventDetail::ActionEvent(FormAction::Exit),
                handled: false
            });
        })
        .build()
    ));

    form.push_component(Box::new(ButtonBuilder::new()
        .set_label("Quit 2")
        .set_position(1, 5)
        .set_action(|q| {
            q.push(Event {
                detail: EventDetail::ActionEvent(FormAction::Exit),
                handled: false
            });
        })
        .build()
    ));

    form.push_component(Box::new(ButtonBuilder::new()
        .set_label("Quit 3")
        .set_position(1, 9)
        .set_neutral_bg_color(COLOR_BLACK)
        .set_focus_bg_color(COLOR_BLACK)
        .set_neutral_fg_color(COLOR_BLUE)
        .set_focus_fg_color(COLOR_RED)
        .set_action(|q| {
            q.push(Event {
                detail: EventDetail::ActionEvent(FormAction::Exit),
                handled: false
            });
        })
        .build()
    ));

    form.run_event_loop();
}
