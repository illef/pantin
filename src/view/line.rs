use super::utils;
use super::*;
use std::fmt::Display;

#[derive(Clone)]
pub struct Line<D: Display + Clone> {
    display: D,
    bg: color::Color,
    fg: color::Color,
    buffer_cache: Option<Buffer>,
    desire_size: Size,
}

impl<D: Display + Clone> View for Line<D> {
    fn desire_size(&self) -> Size {
        self.desire_size
    }
    fn render(&mut self, buf: &mut BufferMut) {
        if self.buffer_cache.is_none() || self.buffer_cache.as_ref().unwrap().size() != buf.size() {
            let mut buffer_cache = Buffer::new(buf.size());
            let iter = utils::str_as_cells(self.display.to_string(), self.bg, self.fg);
            let infinite = utils::make_infinite_cells(' ', self.bg, self.fg);
            let cell_iter = iter.chain(infinite);

            buffer_cache
                .as_mut_view(Point(0, 0), buffer_cache.size())
                .write_cells(cell_iter);

            self.buffer_cache = Some(buffer_cache);
        }
        assert!(self.buffer_cache.is_some());
        assert!(self.buffer_cache.as_ref().unwrap().size() == buf.size());

        buf.write_buffer(self.buffer_cache.as_ref().unwrap());
    }
}

pub fn make_line_view<D: Display + Clone>(
    display: D,
    desire_size: Size,
    bg: color::Color,
    fg: color::Color,
) -> Line<D> {
    Line {
        display,
        bg,
        fg,
        buffer_cache: None,
        desire_size,
    }
}
