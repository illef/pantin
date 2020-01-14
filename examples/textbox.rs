use pantin::*;

mod util;
use util::*;

fn make_view() -> view::DockPanel<BasicEvent> {
    view::make_dock_panel(size(MAX, MAX)).add_child(
        view::Dock::Bottom,
        Box::new(view::make_textbox(
            size(100, 1),
            color::Color::Cyan,
            color::Color::Black,
        )),
    )
}

#[tokio::main]
async fn main() {
    let (event_sender, event_receiver) = mpsc::channel(1024);
    tokio::spawn(async move { send_key_event::<BasicEvent>(event_sender).await });
    run(make_view(), event_receiver).await;
}
