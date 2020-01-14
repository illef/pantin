use pantin::*;
use view::*;

mod util;
use util::*;

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

impl Into<Box<dyn View<Event = BasicEvent>>> for &Person {
    fn into(self) -> Box<dyn View<Event = BasicEvent>> {
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
                    Color::Cyan,
                    Color::Black,
                )),
            );
        Box::new(dock_panel)
    }
}

#[tokio::main]
async fn main() {
    let person = create_person();
    let view = view::make_list_view(person.iter());
    let view = view::make_scroll_viewer::<view::ListView<&Person, BasicEvent>, BasicEvent>(view);

    run(view).await;
}
