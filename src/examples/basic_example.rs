extern crate conui;
use conui::*;
use pancurses::*;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let count = Rc::new(RefCell::<i32>::new(0));
    let mut count_text = TextBinder::new("Count: 0");
    let input_text = TextBinder::new("");

    let mut form = Form::new();
    form.push_component(TextBuilder::new()
        .set_text_binder(count_text.clone())
        .set_position(12, 1)
        .build());

    form.push_component(ButtonBuilder::new()
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
            count_text.set(format!("Count: {}", val).as_str());
        })
        .build()
    );

    form.push_component(TextInputBuilder::new()
        .set_label("Text Input")
        .set_box_width(20)
        .set_position(0, 4)
        .set_text_binder(input_text)
        .build()
    );

    form.push_component(ButtonBuilder::new()
        .set_label("Quit")
        .set_position(0, 8)
        .set_action(|q| {
            q.dispatch_event(EventDetail::ActionEvent(FormAction::Exit));
        })
        .build()
    );

    form.run_event_loop();
}
