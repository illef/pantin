use super::*;
use std::io::prelude::*;

pub struct Screen<V, W> {
    view: V,
    w: W,
    buffer: Buffer,
}

impl<V: View, W: Write> Screen<V, W> {
    pub fn render(&mut self, size: Size) {
        if self.buffer.size() < size {
            self.buffer = Buffer::new(size);
        }
        {
            let mut buffer_mut_view = self.buffer.as_mut_view(Point(0, 0), size);
            self.view.render(&mut buffer_mut_view);
        }

        for y in 0..size.height {
            for x in 0..size.width {
                if let Some(Some(cell)) = self.buffer.get_cell(Point(x, y)) {
                    write!(
                        self.w,
                        "{}{}{}{}",
                        Point(x, y).into_goto(),
                        SetForegroundColor(cell.fg),
                        SetBackgroundColor(cell.bg),
                        cell.ch
                    )
                    .unwrap();
                }
            }
        }

        self.w.flush().unwrap_or_default();
    }

    pub fn desire_size(&self) -> Size {
        self.view.desire_size()
    }
}

pub fn make_screen<V: View, W: Write>(w: W, view: V, initial_size: Size) -> Screen<V, W> {
    Screen {
        w,
        view,
        buffer: Buffer::new(initial_size),
    }
}
