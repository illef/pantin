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

    fn inner_buffer_iter_mut<'a>(&'a mut self) -> impl Iterator<Item = PointWithMutCell<'a>> + '_ {
        let size = self.buffer.size();
        self.buffer
            .buffer
            .iter_mut()
            .enumerate()
            .map(move |(i, cell)| {
                let p = index_into_point(i, size);
                PointWithMutCell { p, cell }
            })
    }

    pub fn as_mut_view<'a>(&'a mut self, offset: Point, size: Size) -> BufferMutView<'a> {
        BufferMutView {
            buffer: self.buffer,
            offset: self.offset.add(offset),
            size,
        }
    }

    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = PointWithMutCell<'a>> + '_ {
        let offset = self.offset;
        let size = self.size;
        self.inner_buffer_iter_mut()
            .filter(move |point_with_cell| {
                point_with_cell.p.0 >= offset.0
                    && point_with_cell.p.0 < (offset.0 + size.width)
                    && point_with_cell.p.1 >= offset.1
                    && point_with_cell.p.1 < (offset.1 + size.height)
            })
            .map(move |mut point_with_cell| {
                point_with_cell.p = point_with_cell.p.sub(offset);
                point_with_cell
            })
    }

    pub fn get_mut_cell(&mut self, p: Point) -> Option<&mut Option<Cell>> {
        let p = p.add(self.offset);
        let size = self.buffer.size();
        if p.is_in(size) {
            Some(&mut self.buffer.buffer[p.into_index(size)])
        } else {
            None
        }
    }

    pub fn write_buffer(&mut self, buffer: &Buffer) {
        buffer.iter().for_each(|point_with_cell| {
            if let Some(dest_cell) = self.get_mut_cell(point_with_cell.p) {
                *dest_cell = *point_with_cell.cell
            }
        });
    }

    pub fn write_cells(&mut self, mut cells: impl Iterator<Item = Cell>) {
        let mut dest = self.iter_mut();

        use unicode_width::UnicodeWidthChar;

        while let Some(point_with_cell) = dest.next() {
            if let Some(cell) = cells.next() {
                let width = cell.ch.width().unwrap() as u16;
                *point_with_cell.cell = Some(cell);
                for _ in 1..width {
                    if let Some(blank_cell) = dest.next() {
                        *blank_cell.cell = None;
                    }
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

    pub fn get_cell(&mut self, p: Point) -> Option<&Option<Cell>> {
        if p.is_in(self.size()) {
            Some(&self.buffer[p.into_index(self.size())])
        } else {
            None
        }
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = PointWithCell<'a>> + '_ {
        let size = self.size;
        self.buffer.iter().enumerate().map(move |(i, cell)| {
            let p = index_into_point(i, size);
            PointWithCell { p, cell }
        })
    }

    //pub fn write(&mut self, p: Point, c: Cell) -> Result<(), BoxError> {
    ////TODO size check
    //self.buffer[p.y as usize * self.size.width as usize + p.x as usize] = Some(c);
    //Ok(())
    //}
    pub fn size(&self) -> Size {
        self.size
    }
}
