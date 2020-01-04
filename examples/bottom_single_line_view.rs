use std::io::{stdout, Read, Write};
use termion::async_stdin;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

use std::time::Duration;

use pantin::*;

fn main() {
    let screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let mut screen = termion::cursor::HideCursor::from(screen);
    let mut keys = async_stdin().keys();
    let mut termion = backend::Termion::new(screen);

    let mut line_view =
        view::make_single_line_view("test", color::Color::Cyan, color::Color::Black);

    loop {
        let key = keys.next();

        if let Some(Ok(key)) = key {
            if key == Key::Char('q') {
                break;
            }
        }

        let size = termion.size();
        let mut buffer_mut_view = termion.get_buffer().as_mut_view(
            Point {
                x: 0,
                y: size.height - 1,
            },
            Size {
                width: size.width,
                height: 1,
            },
        );

        line_view.render(&mut buffer_mut_view);

        termion.update_screen().unwrap();

        std::thread::sleep(Duration::from_millis(1000 / 100));
    }
}
