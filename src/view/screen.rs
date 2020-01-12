use super::*;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
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

impl<V: View + Focusable, W: Write> Focusable for Screen<V, W> {
    fn is_focused(&self) -> bool {
        self.view.is_focused()
    }
    fn set_focus(&mut self, focus: bool) {
        self.view.set_focus(focus);
    }

    //TODO::key j, key k is hard coded, change it.
    fn handle_key_event(&mut self, key: KeyCode) {
        self.view.handle_key_event(key);
    }
}

pub fn make_screen<V: View, W: Write>(w: W, view: V, initial_size: Size) -> Screen<V, W> {
    Screen {
        w,
        view,
        buffer: Buffer::new(initial_size),
    }
}

pub struct AlternateScreen<W: Write> {
    w: W,
}

impl<W: Write> Drop for AlternateScreen<W> {
    fn drop(&mut self) {
        execute!(self.w, LeaveAlternateScreen).unwrap();
    }
}

impl<W: Write> Write for AlternateScreen<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.w.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.w.flush()
    }
}

pub fn make_alternate_screen<W: Write>(mut w: W) -> AlternateScreen<W> {
    execute!(w, EnterAlternateScreen).unwrap();
    AlternateScreen { w }
}

pub struct CursorHidedScreen<W: Write> {
    w: W,
}

impl<W: Write> Drop for CursorHidedScreen<W> {
    fn drop(&mut self) {
        execute!(self.w, crossterm::cursor::Show).unwrap();
    }
}

impl<W: Write> Write for CursorHidedScreen<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.w.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.w.flush()
    }
}

pub fn make_cursor_hided_screen<W: Write>(mut w: W) -> CursorHidedScreen<W> {
    execute!(w, crossterm::cursor::Hide).unwrap();
    CursorHidedScreen { w }
}
