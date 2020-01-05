use super::utils;
use super::*;
use std::fmt::Display;

pub struct SingleLineView<D: Display> {
    display: D,
    bg: color::Color,
    fg: color::Color,
    buffer_cache: Option<Buffer>,
}

impl<D: Display> View for SingleLineView<D> {
    fn desire_size(&self) -> Size {
        Size {
            width: std::u16::MAX,
            height: 1,
        }
    }
    fn render(&mut self, buf: &mut BufferMutView) {
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

pub fn make_single_line_view<D: Display>(
    display: D,
    bg: color::Color,
    fg: color::Color,
) -> SingleLineView<D> {
    SingleLineView {
        display,
        bg,
        fg,
        buffer_cache: None,
    }
}
