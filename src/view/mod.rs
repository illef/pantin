pub mod dock_panel;
pub mod fill;
pub mod list_view;
pub mod screen;
pub mod scroll_viewer;
pub mod stack_panel;
pub mod textblock;
pub mod textbox;

pub use crate::buffer::*;
pub use crate::color::*;
pub use crate::*;
pub use dock_panel::*;
pub use fill::*;
pub use list_view::*;
pub use screen::*;
pub use scroll_viewer::*;
pub use stack_panel::*;
pub use textblock::*;
pub use textbox::*;

use crossterm::event::KeyCode;

pub trait View {
    fn desire_size(&self) -> Size;
    fn render(&mut self, buf: &mut BufferMut);
    fn get_cursor_pos(&self) -> Option<Point> {
        None
    }
}

///Focuable can handle Key Event
pub trait Focusable {
    fn is_focused(&self) -> bool;
    fn set_focus(&mut self, focus: bool);
    fn handle_key_event(&mut self, key: KeyCode);
}

pub fn available_size(available_size: Size, desire_size: Size) -> Size {
    use std::cmp::min;
    size(
        min(available_size.width, desire_size.width),
        min(available_size.height, desire_size.height),
    )
}
