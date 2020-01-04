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
    Size { width, height }
}

impl<W: Write> Termion<W> {
    pub fn new(w: W) -> Termion<W> {
        Termion {
            w,
            buffer: Buffer::new(terminal_size()),
        }
    }

    pub fn get_buffer(&mut self) -> &mut Buffer {
        if terminal_size() != self.buffer.size() {
            self.buffer = Buffer::new(terminal_size())
        }
        &mut self.buffer
    }

    pub fn update_screen(&mut self) -> Result<(), BoxError> {
        let mut iter = self.buffer.iter();
        while let Some(point_with_cell) = iter.next() {
            if let Some(cell) = point_with_cell.cell {
                write!(
                    self.w,
                    "{}{}{}{}",
                    termion::cursor::Goto(point_with_cell.p.x + 1, point_with_cell.p.y + 1),
                    Fg(cell.fg),
                    Bg(cell.bg),
                    cell.ch
                )?;
            }
        }

        self.w.flush()?;

        Ok(())
    }

    pub fn size(&self) -> Size {
        self.buffer.size()
    }
}
