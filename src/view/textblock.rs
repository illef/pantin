use super::utils;
use super::*;
use std::fmt::Display;

#[derive(Clone)]
pub struct TextBlock<D: Display + Clone> {
    display: D,
    bg: color::Color,
    fg: color::Color,
    desire_size: Size,
}

impl<D: Display + Clone> View for TextBlock<D> {
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

pub fn make_textblock<D: Display + Clone>(
    display: D,
    desire_size: Size,
    bg: color::Color,
    fg: color::Color,
) -> TextBlock<D> {
    TextBlock {
        display,
        bg,
        fg,
        desire_size,
    }
}
