use std::io::{stdout, Read, Write};
use termion::async_stdin;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

use std::time::Duration;
use tokio::prelude::*;
use tokio::stream::StreamExt;
use tokio::sync::mpsc;

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

#[derive(Clone, Debug)]
enum Event {
    KeyPressed(Key),
    SizeChanged(Size),
    //AppStop,
}

async fn terminal_size_sender(mut sender: mpsc::Sender<Event>) -> Result<(), error::BoxError> {
    use tokio::time;
    let mut size = terminal_size();

    sender.send(Event::SizeChanged(size)).await?;
    let mut interval = time::interval(Duration::from_millis(100));

    while let Some(_) = interval.next().await {
        let new_size = terminal_size();
        if new_size != size {
            size = new_size;
            sender.send(Event::SizeChanged(size)).await?;
        }
    }
    Ok(())
}

async fn key_event_sender(mut sender: mpsc::Sender<Event>) -> Result<(), error::BoxError> {
    use tokio::time;
    let mut interval = time::interval(Duration::from_millis(10));

    let mut keys = async_stdin().keys();

    while let Some(_) = interval.next().await {
        while let Some(Ok(key)) = keys.next() {
            sender.send(Event::KeyPressed(key)).await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let screen = termion::cursor::HideCursor::from(screen);

    let mut screen = build_view(screen);

    let (event_sender, mut event_receiver) = mpsc::channel(1024);
    let event_sender2 = event_sender.clone();
    tokio::spawn(async move { terminal_size_sender(event_sender2).await });
    tokio::spawn(async move { key_event_sender(event_sender).await });

    while let Some(event) = event_receiver.next().await {
        match event {
            Event::KeyPressed(Key::Char('q')) => break,
            Event::SizeChanged(size) => {
                let mut buffer = Buffer::new(size);
                let mut buffer_mut_view = buffer.as_mut_view(Point(0, 0), buffer.size());

                screen.render(&mut buffer_mut_view);
            }
            _ => {}
        }
    }

    //loop {
    //let key = keys.next();

    //if let Some(Ok(key)) = key {
    //if key == Key::Char('q') {
    //break;
    //}
    //}

    //let mut buffer = Buffer::new(terminal_size());
    //let mut buffer_mut_view = buffer.as_mut_view(Point(0, 0), buffer.size());

    //screen.render(&mut buffer_mut_view);

    //std::thread::sleep(Duration::from_millis(1000 / 100));
    //}
}
