use pantin::*;
use view::*;

mod util;
use util::*;

#[derive(Debug, Clone)]
enum MainEvent {
    Key(KeyEvent),
    Resize(Size),
    SelectedPersonChanged(Option<(usize, Person)>),
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
            MainEvent::SelectedPersonChanged(person) => {
                if let Some(person) = person {
                    self.text_block
                        .set_text(format!("selected_info : {}, {}", person.0, person.1.name));
                } else {
                    self.text_block.set_text("Items is Zero".to_string());
                }
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
#[derive(Debug, Clone, PartialEq)]
struct Person {
    name: String,
    email: String,
}

fn create_person() -> Vec<Person> {
    (1..200)
        .map(|i| Person {
            name: "name".to_owned() + &i.to_string(),
            email: "mail".to_owned() + &i.to_string() + "@example.com",
        })
        .collect()
}

impl AsSelectedListViewItem<MainEvent> for Person {
    fn as_not_selected_view(&self) -> Box<dyn View<Event = MainEvent>> {
        let mut stack_panel = make_stack_panel::<MainEvent>();
        stack_panel
            .add_child(Box::new(make_textblock(
                self.name.clone(),
                size(MAX, 1),
                Color::Reset,
                Color::Reset,
            )))
            .add_child(Box::new(make_textblock(
                "    ".to_owned() + &self.email,
                size(MAX, 1),
                Color::Reset,
                Color::Reset,
            )));
        Box::new(stack_panel)
    }
    fn as_selected_view(&self) -> Box<dyn View<Event = MainEvent>> {
        let mut stack_panel = make_stack_panel::<MainEvent>();
        stack_panel
            .add_child(Box::new(make_textblock(
                self.name.clone(),
                size(MAX, 1),
                Color::DarkGrey,
                Color::White,
            )))
            .add_child(Box::new(make_textblock(
                "    ".to_owned() + &self.email,
                size(MAX, 1),
                Color::DarkGrey,
                Color::Reset,
            )));
        Box::new(stack_panel)
    }
}

fn make_view(mut event_sender: mpsc::Sender<MainEvent>) -> view::DockPanel<MainEvent> {
    let mut list_view = view::make_selectable_list_view(create_person());
    list_view.add_callback(move |selected_info| {
        event_sender
            .try_send(MainEvent::SelectedPersonChanged(selected_info))
            .expect("Always success");
    });

    let text_view = TextView {
        text_block: view::make_textblock(
            "",
            size(MAX, 1),
            color::Color::Green,
            color::Color::Black,
        ),
    };

    view::make_dock_panel(size(MAX, MAX))
        .add_child(view::Dock::Bottom, Box::new(text_view))
        .add_child(view::Dock::Top, Box::new(list_view))
}
#[tokio::main]
async fn main() {
    let (event_sender, event_receiver) = mpsc::channel(1024);
    let view = make_view(event_sender.clone());
    tokio::spawn(send_key_event::<MainEvent>(event_sender));
    run(view, event_receiver).await;
}
