use std::io::{stdout, Read, Write};
use termion::async_stdin;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

use std::time::Duration;

use pantin::*;
use view::*;

struct Person {
    name: String,
    email: String,
}

fn create_person() -> Vec<Person> {
    (1..10)
        .map(|i| Person {
            name: "name".to_owned() + &i.to_string(),
            email: "mail".to_owned() + &i.to_string() + "@example.com",
        })
        .collect()
}

impl Into<Box<dyn View>> for &Person {
    fn into(self) -> Box<dyn View> {
        let mut dock_panel = make_dock_panel(size(MAX, 1));
        dock_panel = dock_panel
            .add_child(
                Dock::Left,
                Box::new(make_line_view(
                    self.name.clone(),
                    size(10, 1),
                    Color::Reset,
                    Color::Reset,
                )),
            )
            .add_child(
                Dock::Left,
                Box::new(make_line_view(
                    self.email.clone(),
                    size(MAX, 1),
                    Color::Reset,
                    Color::Reset,
                )),
            );
        Box::new(dock_panel)
    }
}

fn main() {
    let screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let screen = termion::cursor::HideCursor::from(screen);
    let mut keys = async_stdin().keys();

    let person = create_person();
    let mut screen = view::make_screen(screen, view::make_list_view(person.iter()));

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
