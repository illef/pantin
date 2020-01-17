use pantin::*;
use view::*;

mod util;
use util::*;

#[derive(Clone, PartialEq)]
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

impl AsSelectedListViewItem<BasicEvent> for Person {
    fn as_not_selected_view(&self) -> Box<dyn View<Event = BasicEvent>> {
        let mut dock_panel = make_dock_panel::<BasicEvent>(size(MAX, 1));
        dock_panel = dock_panel
            .add_child(
                Dock::Left,
                Box::new(make_textblock(
                    self.name.clone(),
                    size(10, 1),
                    Color::Black,
                    Color::White,
                )),
            )
            .add_child(
                Dock::Left,
                Box::new(make_textblock(
                    self.email.clone(),
                    size(MAX, 1),
                    Color::Reset,
                    Color::Reset,
                )),
            );
        Box::new(dock_panel)
    }
    fn as_selected_view(&self) -> Box<dyn View<Event = BasicEvent>> {
        let mut dock_panel = make_dock_panel::<BasicEvent>(size(MAX, 1));
        dock_panel = dock_panel
            .add_child(
                Dock::Left,
                Box::new(make_textblock(
                    self.name.clone(),
                    size(10, 1),
                    Color::Cyan,
                    Color::Black,
                )),
            )
            .add_child(
                Dock::Left,
                Box::new(make_textblock(
                    self.email.clone(),
                    size(MAX, 1),
                    Color::Reset,
                    Color::Reset,
                )),
            );
        Box::new(dock_panel)
    }
}

#[tokio::main]
async fn main() {
    let view = view::make_selectable_list_view(create_person());

    let (event_sender, event_receiver) = mpsc::channel(1024);
    tokio::spawn(send_key_event::<BasicEvent>(event_sender));
    run(view, event_receiver).await;
}
