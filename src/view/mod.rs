pub mod single_line_view;

pub use crate::buffer::*;
pub use crate::color::*;
pub use crate::*;
pub use single_line_view::*;

pub trait View {
    fn desire_size(&self) -> Size;
    fn render(&mut self, buf: &mut BufferMutView);
}
