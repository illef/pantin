use super::utils;
use super::*;
use std::fmt::Display;

pub struct SingleLineView<D: Display> {
    display: D,
    bg: color::Color,
    fg: color::Color,
}

impl<D: Display> View for SingleLineView<D> {
    fn desire_size(&self) -> Size {
        Size {
            width: std::u16::MAX,
            height: 1,
        }
    }
    fn render(&mut self, buf: &mut BufferMutView) {
        let iter = utils::str_as_cells(self.display.to_string(), self.bg, self.fg);
        let infinite = utils::make_infinite_cells(' ', self.bg, self.fg);
        let cell_iter = iter.chain(infinite);

        buf.write_cells(cell_iter);
    }
}

pub fn make_single_line_view<D: Display>(
    display: D,
    bg: color::Color,
    fg: color::Color,
) -> SingleLineView<D> {
    SingleLineView { display, bg, fg }
}
