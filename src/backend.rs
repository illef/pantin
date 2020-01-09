use crate::buffer::*;
use crate::error::*;
use crate::*;
use std::io::prelude::*;

pub struct Termion<W: Write> {
    w: W,
    buffer: Buffer,
}

fn terminal_size() -> Size {
    let (width, height) = termion::terminal_size().unwrap();
    size(width, height)
}

impl<W: Write> Termion<W> {
    pub fn new(w: W) -> Termion<W> {
        Termion {
            w,
            buffer: Buffer::new(terminal_size()),
        }
    }

    pub fn get_buffer_view(&mut self) -> BufferMut {
        if terminal_size() != self.buffer.size() {
            self.buffer = Buffer::new(terminal_size())
        }
        self.buffer.as_mut_view(Point(0, 0), self.size())
    }

    pub fn update_screen(&mut self) -> Result<(), BoxError> {
        let size = self.size();
        let mut iter = self.buffer.iter();
        while let Some(point_with_cell) = iter.next() {
            if point_with_cell.p.is_in(size) {
                if let Some(cell) = point_with_cell.cell {
                    write!(
                        self.w,
                        "{}{}{}{}",
                        point_with_cell.p.into_goto(),
                        Fg(cell.fg),
                        Bg(cell.bg),
                        cell.ch
                    )?;
                }
            }
        }

        self.w.flush()?;

        Ok(())
    }

    pub fn size(&self) -> Size {
        self.buffer.size()
    }
}
