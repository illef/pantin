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

use crossterm::event::{self, KeyCode};

pub trait AsUIEvent: Clone + std::fmt::Debug + Sync + Send {
    fn as_ui_event(&self) -> Option<event::Event>;
    fn from_tui_event(e: crossterm::event::Event) -> Self;
}

pub trait View {
    type Event: AsUIEvent;
    fn desire_size(&self) -> Size;
    fn render(&mut self, buf: &mut BufferMut);

    //for focusable
    fn is_focusable(&self) -> bool {
        false
    }
    fn is_focused(&self) -> bool {
        false
    }
    fn set_focus(&mut self, _: bool) {}
    fn handle_key_event(&mut self, _: KeyCode) {}
}

pub fn available_size(available_size: Size, desire_size: Size) -> Size {
    use std::cmp::min;
    size(
        min(available_size.width, desire_size.width),
        min(available_size.height, desire_size.height),
    )
}
