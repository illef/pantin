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

    loop {
        let key = keys.next();

        if let Some(Ok(key)) = key {
            if key == Key::Char('q') {
                break;
            }
        }

        std::thread::sleep(Duration::from_millis(1000 / 100));
    }
}
