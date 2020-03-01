extern crate conui;
use conui::*;

#[tokio::main]
async fn main() {
    let mut form = Form::new();
    let btn = Button::new();

    form.push_component(Box::new(btn));

    form.run_event_loop().await;
}
