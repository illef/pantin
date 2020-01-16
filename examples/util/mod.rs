pub use crossterm::event::Event;
pub use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
pub use pantin::view::*;
use std::io::stdout;
pub use tokio::stream::StreamExt;
pub use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub enum BasicEvent {
    Key(KeyEvent),
    Resize(Size),
    Other,
}

impl AsUIEvent for BasicEvent {
    fn as_ui_event(&self) -> Option<Event> {
        match self {
            BasicEvent::Key(key_event) => Some(Event::Key(*key_event)),
            BasicEvent::Resize(size) => Some(Event::Resize(size.width, size.height)),
            _ => None,
        }
    }
    fn from_tui_event(e: Event) -> Self {
        match e {
            Event::Key(key_event) => BasicEvent::Key(key_event),
            Event::Resize(w, h) => BasicEvent::Resize(size(w, h)),
            _ => BasicEvent::Other,
        }
    }
}

pub async fn send_key_event<E: AsUIEvent + 'static>(
    mut sender: mpsc::Sender<E>,
) -> Result<(), error::BoxError> {
    let mut event_stream = crossterm::event::EventStream::new();
    while let Some(Ok(event)) = event_stream.next().await {
        sender.send(E::from_tui_event(event)).await?;
    }

    Ok(())
}

pub async fn run<E: AsUIEvent + 'static, V: View<Event = E>>(
    view: V,
    mut event_receiver: mpsc::Receiver<E>,
) {
    crossterm::terminal::enable_raw_mode().unwrap();

    let screen = make_alternate_screen(stdout());
    let screen = make_cursor_hided_screen(screen);

    let mut screen = make_screen(screen, view, terminal_size());
    screen.render(terminal_size());

    while let Some(event) = event_receiver.next().await {
        if screen.apply_event(&event) {
            screen.render(terminal_size());
        } else {
            match event.as_ui_event() {
                Some(Event::Key(key_event)) => {
                    if key_event.code == KeyCode::Char('c')
                        && key_event.modifiers == KeyModifiers::CONTROL
                    {
                        break;
                    } else if screen.is_focused() {
                        screen.handle_key_event(key_event.code);
                        screen.render(terminal_size());
                    }
                }
                Some(Event::Resize(width, height)) => {
                    screen.render(size(width, height));
                }
                _ => {}
            }
        }
    }
}
