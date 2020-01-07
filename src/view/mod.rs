pub mod dock_panel;
pub mod fill;
pub mod line_view;

pub use crate::buffer::*;
pub use crate::color::*;
pub use crate::*;
pub use dock_panel::*;
pub use fill::*;
pub use line_view::*;

pub trait View {
    fn desire_size(&self) -> Size;
    fn render(&mut self, buf: &mut BufferMutView);
}
