use super::*;
use std::io::prelude::*;

pub struct Screen<V, W> {
    view: V,
    w: W,
}

impl<V: View, W: Write> View for Screen<V, W> {
    fn desire_size(&self) -> Size {
        self.view.desire_size()
    }
    fn render(&mut self, buf: &mut BufferMut) {
        self.view.render(buf);

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

        self.w.flush().unwrap_or_default();
    }
}

pub fn make_screen<V: View, W: Write>(w: W, view: V) -> Screen<V, W> {
    Screen { w, view }
}
