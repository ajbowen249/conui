extern crate conui;
use conui::*;

#[tokio::main]
async fn main() {
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
        .set_position(1, 2)
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
        .set_position(1, 3)
        .set_action(|q| {
            q.push(Event {
                detail: EventDetail::ActionEvent(FormAction::Exit),
                handled: false
            });
        })
        .build()
    ));


    form.run_event_loop().await;
}
