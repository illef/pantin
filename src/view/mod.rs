pub mod dock_panel;
pub mod fill;
pub mod line;
pub mod list_view;
pub mod stack_panel;

pub use crate::buffer::*;
pub use crate::color::*;
pub use crate::*;
pub use dock_panel::*;
pub use fill::*;
pub use line::*;
pub use list_view::*;
pub use stack_panel::*;

pub trait View {
    fn desire_size(&self) -> Size;
    fn render(&mut self, buf: &mut BufferMut);
}

pub fn available_width(available_size: Size, desire_size: Size) -> u16 {
    if available_size.width < desire_size.width {
        available_size.width
    } else {
        desire_size.width
    }
}

pub fn available_height(available_size: Size, desire_size: Size) -> u16 {
    if available_size.height < desire_size.height {
        available_size.height
    } else {
        desire_size.height
    }
}
