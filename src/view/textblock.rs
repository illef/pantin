use super::utils;
use super::*;
use std::fmt::Display;

#[derive(Clone)]
pub struct TextBlock<D: Display + Clone, E: AsUIEvent> {
    display: D,
    bg: color::Color,
    fg: color::Color,
    desire_size: Size,
    phantom: std::marker::PhantomData<E>,
}

impl<D: Display + Clone, E: AsUIEvent> View for TextBlock<D, E> {
    type Event = E;
    fn desire_size(&self) -> Size {
        self.desire_size
    }
    fn render(&mut self, buf: &mut BufferMut) {
        let iter = utils::str_as_cells(self.display.to_string(), self.bg, self.fg);
        let infinite = utils::make_infinite_cells(' ', self.bg, self.fg);
        let cell_iter = iter.chain(infinite);

        buf.write_cells(cell_iter);
    }
}

pub fn make_textblock<D: Display + Clone, E: AsUIEvent>(
    display: D,
    desire_size: Size,
    bg: color::Color,
    fg: color::Color,
) -> TextBlock<D, E> {
    TextBlock {
        display,
        bg,
        fg,
        desire_size,
        phantom: std::marker::PhantomData,
    }
}
