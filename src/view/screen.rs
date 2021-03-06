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
    cache_buffer: Buffer,
}

impl<V: View, W: Write> Screen<V, W> {
    pub fn render(&mut self, size: Size) {
        let size = available_size(size, self.desire_size());
        if self.buffer.size() < size {
            self.buffer = Buffer::new(size * 2);
            self.cache_buffer = self.buffer.clone();
        }
        {
            let mut buffer_mut_view = self.buffer.as_mut_view(Point(0, 0), size);
            self.view.render(&mut buffer_mut_view);
        }

        let mut temp_buffer = Buffer::make_empty();
        let mut temp_cache_buffer = temp_buffer.clone();

        std::mem::swap(&mut temp_buffer, &mut self.buffer);
        std::mem::swap(&mut temp_cache_buffer, &mut self.cache_buffer);

        let mut cursor_on_cell = None;

        temp_buffer
            .get_diff(&temp_cache_buffer, size)
            .for_each(|point_with_cell| {
                if let Some(cell) = point_with_cell.cell {
                    write!(
                        self.w,
                        "{}{}{}{}",
                        point_with_cell.p.into_goto(),
                        SetForegroundColor(cell.fg),
                        SetBackgroundColor(cell.bg),
                        cell.ch
                    )
                    .unwrap();
                    if cell.cursor_on {
                        cursor_on_cell = Some((point_with_cell.p, *cell))
                    }
                }
            });

        std::mem::swap(&mut temp_buffer, &mut self.cache_buffer);
        std::mem::swap(&mut temp_cache_buffer, &mut self.buffer);

        //cursor position
        if let Some(cursor_pos) = cursor_on_cell {
            write!(self.w, "{}", crossterm::cursor::Show).unwrap();
            write!(self.w, "{}", cursor_pos.0.into_goto()).unwrap();
        } else {
            write!(self.w, "{}", crossterm::cursor::Hide).unwrap();
        }
        self.w.flush().unwrap_or_default();
    }

    pub fn apply_event(&mut self, event: &<V as view::View>::Event) -> bool {
        self.view.apply_event(event)
    }
    pub fn desire_size(&self) -> Size {
        self.view.desire_size()
    }

    pub fn is_focusable(&self) -> bool {
        self.view.is_focusable()
    }

    pub fn is_focused(&self) -> bool {
        self.view.is_focused()
    }

    pub fn set_focus(&mut self, focus: bool) {
        self.view.set_focus(focus);
    }

    pub fn handle_key_event(&mut self, key: KeyCode) {
        self.view.handle_key_event(key);
    }
}

pub fn make_screen<V: View, W: Write>(w: W, view: V, initial_size: Size) -> Screen<V, W> {
    Screen {
        w,
        view,
        buffer: Buffer::new(initial_size * 2),
        cache_buffer: Buffer::new(initial_size * 2),
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
