use pantin::*;
use view::*;

mod util;
use util::*;

static mut index: usize = 0;

fn get_color() -> color::Color {
    let color = unsafe {
        match index % 3 {
            0 => color::Color::Yellow,
            1 => color::Color::Blue,
            2 => color::Color::Red,
            _ => unreachable!(),
        }
    };

    unsafe {
        index += 1;
    };
    color
}

fn make_dock_panel(dock_panel: DockPanel<BasicEvent>) -> DockPanel<BasicEvent> {
    dock_panel
        .add_child(Dock::Left, Box::new(make_fill(get_color(), size(1, MAX))))
        .add_child(Dock::Top, Box::new(make_fill(get_color(), size(MAX, 1))))
        .add_child(Dock::Right, Box::new(make_fill(get_color(), size(1, MAX))))
        .add_child(Dock::Bottom, Box::new(make_fill(get_color(), size(MAX, 1))))
}

fn build_view() -> DockPanel<BasicEvent> {
    let mut dock_panel = view::make_dock_panel(size(MAX, MAX));

    for _ in 0..100 {
        dock_panel = make_dock_panel(dock_panel);
    }
    dock_panel
}

#[tokio::main]
async fn main() {
    let (event_sender, event_receiver) = mpsc::channel(1024);
    tokio::spawn(async move { send_key_event::<BasicEvent>(event_sender).await });
    run(build_view(), event_receiver).await;
}
