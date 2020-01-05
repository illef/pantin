use std::io::{stdout, Read, Write};
use termion::async_stdin;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

use std::time::Duration;

use pantin::*;
use view::*;

fn main() {
    let screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let screen = termion::cursor::HideCursor::from(screen);
    let mut keys = async_stdin().keys();
    let mut termion = backend::Termion::new(screen);

    let mut dock_panel = view::make_dock_panel();

    dock_panel.add_child(
        Alignment::Bottom,
        Box::new(view::make_single_line_view(
            "footer",
            color::Color::Cyan,
            color::Color::Black,
        )),
    );

    dock_panel.add_child(
        Alignment::Top,
        Box::new(view::make_single_line_view(
            "header",
            color::Color::Cyan,
            color::Color::Black,
        )),
    );

    loop {
        let key = keys.next();

        if let Some(Ok(key)) = key {
            if key == Key::Char('q') {
                break;
            }
        }

        let size = termion.size();
        let mut termion_buffer_view = termion.get_buffer_view();

        let mut buffer_mut_view = termion_buffer_view.as_mut_view(Point(0, 0), size);

        dock_panel.render(&mut buffer_mut_view);

        termion.update_screen().unwrap();

        std::thread::sleep(Duration::from_millis(1000 / 100));
    }
}
