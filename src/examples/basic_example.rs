extern crate conui;
use conui::*;
use pancurses::*;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let mut form = Form::new();

    let count_label = new_component_ref(TextBuilder::new()
        .set_text("Count: 0")
        .set_position(12, 1)
        .build());

    form.push_component(count_label.clone());

    let count = Rc::new(RefCell::<i32>::new(0));

    form.push_component(new_component_ref(ButtonBuilder::new()
        .set_label("Increment")
        .set_position(0, 0)
        .set_neutral_bg_color(COLOR_BLACK)
        .set_focus_bg_color(COLOR_BLACK)
        .set_neutral_fg_color(COLOR_BLUE)
        .set_focus_fg_color(COLOR_RED)
        .set_action(move |_| {
            let mut val = *count.borrow();
            val = val + 1;
            *count.borrow_mut() = val;
            count_label.borrow_mut().set_text(format!("Count: {}", val).as_str());
        })
        .build()
    ));

    form.push_component(new_component_ref(ButtonBuilder::new()
        .set_label("Quit")
        .set_position(0, 4)
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
