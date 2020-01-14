use crossterm::event::Event;
use crossterm::event::KeyCode;
use pantin::view::*;
use std::io::stdout;
use tokio::stream::StreamExt;
use tokio::sync::mpsc;

async fn send_key_event(mut sender: mpsc::Sender<Event>) -> Result<(), error::BoxError> {
    let mut event_stream = crossterm::event::EventStream::new();
    while let Some(Ok(event)) = event_stream.next().await {
        sender.send(event).await?;
    }

    Ok(())
}

pub async fn run<V: View>(view: V) {
    crossterm::terminal::enable_raw_mode().unwrap();

    let screen = make_alternate_screen(stdout());
    let screen = make_cursor_hided_screen(screen);

    let mut screen = make_screen(screen, view, terminal_size());
    screen.render(terminal_size());

    let (event_sender, mut event_receiver) = mpsc::channel(1024);
    tokio::spawn(async move { send_key_event(event_sender).await });

    while let Some(event) = event_receiver.next().await {
        match event {
            Event::Key(key_event) => {
                if key_event.code == KeyCode::Char('q') {
                    break;
                } else if screen.is_focused() {
                    screen.handle_key_event(key_event.code);
                    screen.render(terminal_size());
                }
            }
            Event::Resize(width, height) => {
                screen.render(size(width, height));
            }
            _ => {}
        }
    }
}
