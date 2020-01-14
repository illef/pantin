pub use crossterm::event::Event;
pub use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use pantin::view::*;
use std::io::stdout;
use tokio::stream::StreamExt;
use tokio::sync::mpsc;

#[derive(Debug)]
pub enum BasicEvent {
    Key(KeyEvent),
    Resize(Size),
    Other,
}

impl AsKeyEvent for BasicEvent {
    fn as_key_event(&self) -> Option<KeyEvent> {
        match self {
            BasicEvent::Key(key_event) => Some(*key_event),
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

async fn send_key_event(mut sender: mpsc::Sender<BasicEvent>) -> Result<(), error::BoxError> {
    let mut event_stream = crossterm::event::EventStream::new();
    while let Some(Ok(event)) = event_stream.next().await {
        sender.send(BasicEvent::from_tui_event(event)).await?;
    }

    Ok(())
}

pub async fn run<E: AsKeyEvent, V: View<Event = E>>(view: V) {
    crossterm::terminal::enable_raw_mode().unwrap();

    let screen = make_alternate_screen(stdout());
    let screen = make_cursor_hided_screen(screen);

    let mut screen = make_screen(screen, view, terminal_size());
    screen.render(terminal_size());

    let (event_sender, mut event_receiver) = mpsc::channel(1024);
    tokio::spawn(async move { send_key_event(event_sender).await });

    while let Some(event) = event_receiver.next().await {
        match event {
            BasicEvent::Key(key_event) => {
                if key_event.code == KeyCode::Char('c')
                    && key_event.modifiers == KeyModifiers::CONTROL
                {
                    break;
                } else if screen.is_focused() {
                    screen.handle_key_event(key_event.code);
                    screen.render(terminal_size());
                }
            }
            BasicEvent::Resize(size) => {
                screen.render(size);
            }
            _ => {}
        }
    }
}
