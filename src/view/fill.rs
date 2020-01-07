use super::utils;
use super::*;
use std::fmt::Display;

pub struct Fill {
    bg: color::Color,
    size: Size,
}

impl View for Fill {
    fn desire_size(&self) -> Size {
        self.size
    }
    fn render(&mut self, buf: &mut BufferMutView) {
        let cells = utils::make_infinite_cells(' ', self.bg, color::Color::Reset);
        buf.write_cells(cells);
    }
}

pub fn make_fill<D: Display>(bg: color::Color, size: Size) -> Fill {
    Fill { bg, size }
}
