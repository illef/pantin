use super::utils;
use super::*;
use std::io::prelude::*;

pub struct Screen<'a, W> {
    inner: Box<dyn View + 'a>,
    w: W,
}

impl<'a, W: Write> View for Screen<'a, W> {
    fn desire_size(&self) -> Size {
        self.inner.desire_size()
    }
    fn render(&mut self, buf: &mut BufferMut) {
        self.inner.render(buf);

        for y in 0..buf.size().height {
            for x in 0..buf.size().width {
                if let Some(Some(cell)) = buf.get_cell(Point(x, y)) {
                    write!(
                        self.w,
                        "{}{}{}{}",
                        Point(x, y).into_goto(),
                        Fg(cell.fg),
                        Bg(cell.bg),
                        cell.ch
                    )
                    .unwrap();
                }
            }
        }

        self.w.flush();
    }
}

pub fn make_screen<'a, W: Write>(w: W, inner: Box<dyn View + 'a>) -> Screen<'a, W> {
    Screen { w, inner }
}
