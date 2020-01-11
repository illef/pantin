use std::io::{stdout, Read, Write};
use termion::async_stdin;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

use std::time::Duration;

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

fn main() {
    let screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let screen = termion::cursor::HideCursor::from(screen);
    let mut keys = async_stdin().keys();

    let mut screen = {
        let mut dock_panel = view::make_dock_panel(size(MAX, MAX));

        for _ in 0..100 {
            dock_panel = make_dock_panel(dock_panel);
        }
        view::make_screen(screen, Box::new(dock_panel))
    };

    loop {
        let key = keys.next();

        if let Some(Ok(key)) = key {
            if key == Key::Char('q') {
                break;
            }
        }

        let mut buffer = Buffer::new(terminal_size());
        let mut buffer_mut_view = buffer.as_mut_view(Point(0, 0), buffer.size());

        screen.render(&mut buffer_mut_view);

        std::thread::sleep(Duration::from_millis(1000 / 100));
    }
}
