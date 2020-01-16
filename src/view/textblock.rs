use super::utils;
use super::*;
use std::fmt::Display;

#[derive(Clone)]
pub struct TextBlock<E: AsUIEvent> {
    string: String,
    bg: color::Color,
    fg: color::Color,
    desire_size: Size,
    phantom: std::marker::PhantomData<E>,
}

impl<E: AsUIEvent> TextBlock<E> {
    pub fn set_text(&mut self, text: String) {
        self.string = text;
    }
}

impl<E: AsUIEvent> View for TextBlock<E> {
    type Event = E;
    fn desire_size(&self) -> Size {
        self.desire_size
    }
    fn render(&mut self, buf: &mut BufferMut) {
        let iter = utils::str_as_cells(&self.string, self.bg, self.fg);
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
) -> TextBlock<E> {
    TextBlock {
        string: display.to_string(),
        bg,
        fg,
        desire_size,
        phantom: std::marker::PhantomData,
    }
}
