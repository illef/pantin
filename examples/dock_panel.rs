use std::io::{stdout, Write};

use tokio::stream::StreamExt;
use tokio::sync::mpsc;

use crossterm::event::Event;
use crossterm::event::{KeyCode, KeyEvent};
use pantin::*;
use view::*;

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

fn make_dock_panel(dock_panel: DockPanel) -> DockPanel {
    dock_panel
        .add_child(Dock::Left, Box::new(make_fill(get_color(), size(1, MAX))))
        .add_child(Dock::Top, Box::new(make_fill(get_color(), size(MAX, 1))))
        .add_child(Dock::Right, Box::new(make_fill(get_color(), size(1, MAX))))
        .add_child(Dock::Bottom, Box::new(make_fill(get_color(), size(MAX, 1))))
}

fn build_view(w: impl Write) -> Screen<impl View, impl Write> {
    let mut dock_panel = view::make_dock_panel(size(MAX, MAX));

    for _ in 0..100 {
        dock_panel = make_dock_panel(dock_panel);
    }
    view::make_screen(w, dock_panel)
}

async fn send_key_event(mut sender: mpsc::Sender<Event>) -> Result<(), error::BoxError> {
    let mut event_stream = crossterm::event::EventStream::new();
    while let Some(Ok(event)) = event_stream.next().await {
        sender.send(event).await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    use crossterm::{
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    };

    crossterm::terminal::enable_raw_mode().unwrap();
    execute!(stdout(), EnterAlternateScreen).unwrap();
    execute!(stdout(), crossterm::cursor::Hide).unwrap();

    let mut screen = build_view(stdout());
    screen.draw(terminal_size());

    let (event_sender, mut event_receiver) = mpsc::channel(1024);
    tokio::spawn(async move { send_key_event(event_sender).await });

    while let Some(event) = event_receiver.next().await {
        match event {
            Event::Key(key_event) => {
                if key_event.code == KeyCode::Char('q') {
                    break;
                }
            }
            Event::Resize(width, height) => {
                screen.draw(size(width, height));
            }
            _ => {}
        }
    }

    execute!(stdout(), LeaveAlternateScreen).unwrap();
    execute!(stdout(), crossterm::cursor::Show).unwrap();
}
