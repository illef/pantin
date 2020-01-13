use pantin::*;

mod util;
use util::*;

#[tokio::main]
async fn main() {
    run_focusable_view(view::make_textbox(
        size(100, 1),
        color::Color::Cyan,
        color::Color::Black,
    ))
    .await;
}
