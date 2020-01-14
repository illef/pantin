use super::utils;
use super::*;

#[derive(Clone)]
pub struct Fill<E: AsKeyEvent> {
    bg: color::Color,
    size: Size,
    phantom: std::marker::PhantomData<E>,
}

impl<E: AsKeyEvent> View for Fill<E> {
    type Event = E;
    fn desire_size(&self) -> Size {
        self.size
    }
    fn render(&mut self, buf: &mut BufferMut) {
        let cells = utils::make_infinite_cells(' ', self.bg, self.bg);
        buf.write_cells(cells);
    }
}

pub fn make_fill<E: AsKeyEvent>(bg: color::Color, size: Size) -> Fill<E> {
    Fill {
        bg,
        size,
        phantom: std::marker::PhantomData,
    }
}
