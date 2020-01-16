use pantin::*;

mod util;
use util::*;

#[derive(Debug, Clone)]
pub enum MainEvent {
    Key(KeyEvent),
    Resize(Size),
    TextChanged(String),
    Other,
}

impl AsUIEvent for MainEvent {
    fn as_ui_event(&self) -> Option<Event> {
        match self {
            Self::Key(key_event) => Some(Event::Key(*key_event)),
            Self::Resize(size) => Some(Event::Resize(size.width, size.height)),
            _ => None,
        }
    }
    fn from_tui_event(e: Event) -> Self {
        match e {
            Event::Key(key_event) => Self::Key(key_event),
            Event::Resize(w, h) => Self::Resize(size(w, h)),
            _ => Self::Other,
        }
    }
}

struct TextView {
    text_block: TextBlock<MainEvent>,
}

impl View for TextView {
    type Event = MainEvent;

    fn apply_event(&mut self, event: &MainEvent) -> bool {
        match event {
            MainEvent::TextChanged(string) => {
                self.text_block.set_text(string.clone());
                true
            }
            _ => false,
        }
    }

    fn desire_size(&self) -> Size {
        self.text_block.desire_size()
    }
    fn render(&mut self, buf: &mut BufferMut) {
        self.text_block.render(buf);
    }
}

fn make_view(mut event_sender: mpsc::Sender<MainEvent>) -> view::DockPanel<MainEvent> {
    let mut text_box = view::make_textbox(size(MAX, 1), color::Color::Cyan, color::Color::Black);
    text_box.add_callback(move |text: String| {
        event_sender
            .try_send(MainEvent::TextChanged(text))
            .expect("Always success");
    });

    let text_view = TextView {
        text_block: view::make_textblock("", size(MAX, 1), color::Color::Red, color::Color::White),
    };

    view::make_dock_panel(size(MAX, MAX))
        .add_child(view::Dock::Top, Box::new(text_view))
        .add_child(view::Dock::Bottom, Box::new(text_box))
}

#[tokio::main]
async fn main() {
    let (event_sender, event_receiver) = mpsc::channel(1);
    let view = make_view(event_sender.clone());
    tokio::spawn(send_key_event::<MainEvent>(event_sender));
    run(view, event_receiver).await;
}
