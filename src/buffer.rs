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

impl BufferMutView<'_> {
    pub fn size(&self) -> Size {
        self.size
    }

    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = PointWithMutCell<'a>> + '_ {
        let offset = self.offset;
        let size = self.size;
        self.buffer.iter_mut().filter(move |point_with_cell| {
            point_with_cell.p.x >= offset.x
                && point_with_cell.p.x < (offset.x + size.width)
                && point_with_cell.p.y >= offset.y
                && point_with_cell.p.y < (offset.y + size.height)
        })
    }

    pub fn write_cells(&mut self, mut cells: impl Iterator<Item = Cell>) {
        let mut dest = self.iter_mut();

        use unicode_width::UnicodeWidthChar;

        while let Some(point_with_cell) = dest.next() {
            if let Some(cell) = cells.next() {
                let width = cell.ch.width().unwrap() as u16;
                *point_with_cell.cell = Some(cell);
                for _ in 1..width {
                    let _ = dest.next();
                }
            } else {
                break;
            }
        }
    }
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

    pub fn write(&mut self, p: Point, c: Cell) -> Result<(), BoxError> {
        //TODO size check
        self.buffer[p.y as usize * self.size.width as usize + p.x as usize] = Some(c);
        Ok(())
    }
    pub fn size(&self) -> Size {
        self.size
    }
}
