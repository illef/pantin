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

fn make_stack_panel(stack_panel: StackPanel<BasicEvent>) -> StackPanel<BasicEvent> {
    stack_panel
        .add_child(Box::new(make_fill(get_color(), size(MAX, 5))))
        .add_child(Box::new(make_fill(get_color(), size(MAX, 5))))
        .add_child(Box::new(make_fill(get_color(), size(MAX, 5))))
        .add_child(Box::new(make_fill(get_color(), size(MAX, 5))))
}

#[tokio::main]
async fn main() {
    let mut stack_panel = view::make_stack_panel();

    for _ in 0..100 {
        stack_panel = make_stack_panel(stack_panel);
    }

    let (event_sender, event_receiver) = mpsc::channel(1024);
    tokio::spawn(async move { send_key_event::<BasicEvent>(event_sender).await });
    run(make_stack_panel(stack_panel), event_receiver).await;
}
