use crate::buffer::*;
use crate::*;

pub trait View {
    fn desire_size(&self) -> Size;
    fn render(&mut self, buf: &mut BufferMutView);
}
