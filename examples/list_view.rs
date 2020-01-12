use pantin::*;
use view::*;

mod util;
use util::*;

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
                Box::new(make_textblock(
                    self.name.clone(),
                    size(10, 1),
                    Color::Reset,
                    Color::Reset,
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
    let person = create_person();
    let view = view::make_list_view(person.iter());

    run(view).await;
}
