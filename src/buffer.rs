use crate::error::*;
use crate::*;

pub struct Buffer {
    buffer: Vec<Option<Cell>>,
    size: Size,
}

pub struct BufferMutView<'a> {
    buffer: &'a mut Buffer,
    size: Size,
    offset: Point,
}

impl Buffer {
    pub fn new(size: Size) -> Buffer {
        Buffer {
            size,
            buffer: vec![None; size.width as usize * size.height as usize],
        }
    }

    pub fn as_mut_view<'a>(&'a mut self, offset: Point, size: Size) -> BufferMutView<'a> {
        BufferMutView {
            buffer: self,
            offset,
            size,
        }
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = PointWithCell<'a>> + '_ {
        let width = self.size.width;
        self.buffer.iter().enumerate().map(move |(i, cell)| {
            let p = Point {
                y: i as u16 / width,
                x: i as u16 % width,
            };
            PointWithCell {
                p,
                cell: cell.as_ref(),
            }
        })
    }

    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = PointWithMutCell<'a>> + '_ {
        let width = self.size.width;
        self.buffer.iter_mut().enumerate().map(move |(i, cell)| {
            let p = Point {
                y: i as u16 / width,
                x: i as u16 % width,
            };
            PointWithMutCell { p, cell }
        })
    }

    fn write(&mut self, p: Point, c: Cell) -> Result<(), BoxError> {
        //TODO size check
        self.buffer[p.y as usize * self.size.width as usize + p.x as usize] = Some(c);
        Ok(())
    }
    fn size(&self) -> Size {
        self.size.clone()
    }
}
