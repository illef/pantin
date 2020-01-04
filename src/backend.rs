use crate::error::*;
use crate::*;
use std::io::prelude::*;

pub struct Termion<W: Write> {
    w: W,
}

impl<W: Write> Termion<W> {
    pub fn new(w: W) -> Termion<W> {
        Termion { w }
    }

    fn write(&mut self, c: PointWithCell) -> Result<(), BoxError> {
        if let Some(cell) = c.cell {
            write!(
                self.w,
                "{}{}{}{}",
                termion::cursor::Goto(c.p.x + 1, c.p.y + 1),
                Fg(cell.fg),
                Bg(cell.bg),
                cell.ch
            )?;
        }
        Ok(())
    }

    fn size(&self) -> Size {
        let (width, height) = termion::terminal_size().unwrap();
        Size { width, height }
    }
}
