use crate::error::*;
use crate::*;

#[derive(Clone)]
pub struct Buffer {
    buffer: Vec<Option<Cell>>,
    size: Size,
}

pub struct BufferMut<'a> {
    buffer: &'a mut Buffer,
    size: Size,
    offset: Point,
}

impl BufferMut<'_> {
    pub fn size(&self) -> Size {
        self.size
    }

    pub fn as_mut_view<'a>(&'a mut self, offset: Point, size: Size) -> BufferMut<'a> {
        BufferMut {
            buffer: self.buffer,
            offset: self.offset.add(offset),
            size,
        }
    }

    pub fn get_cell(&self, p: Point) -> Option<&Option<Cell>> {
        let p = p.add(self.offset);
        let size = self.buffer.size();
        if p.is_in(size) {
            Some(&self.buffer.buffer[p.into_index(size)])
        } else {
            None
        }
    }

    pub fn write_cell(&mut self, p: Point, cell: Option<Cell>) {
        let p = p.add(self.offset);
        let size = self.buffer.size();
        if p.is_in(size) {
            self.buffer.buffer[p.into_index(size)] = cell;
        } else {
            panic!("outbound write_cell");
        }
    }

    pub fn write_cells(&mut self, mut cells: impl Iterator<Item = Cell>) {
        use unicode_width::UnicodeWidthChar;
        let mut index = 0;

        let mut write_cell = |index: usize, cell: Option<Cell>| -> bool {
            let point = index_into_point(index, self.size);
            if point.is_in(self.size) {
                self.write_cell(point, cell);
                true
            } else {
                false
            }
        };

        while let Some(cell) = cells.next() {
            let width = cell.ch.width().unwrap() as u16;
            if write_cell(index, Some(cell)) == false {
                break;
            }
            for _ in 1..width {
                index += 1;
                if write_cell(index, None) == false {
                    break;
                }
            }
            index += 1;
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

    pub fn make_empty() -> Buffer {
        Buffer {
            size: size(0, 0),
            buffer: vec![],
        }
    }

    pub fn set_bg(mut self, bg: Option<color::Color>) -> Self {
        if let Some(bg) = bg {
            let bg_iter = utils::make_infinite_cells(' ', bg, color::Color::Reset);
            let size = self.size();
            self.as_mut_view(Point(0, 0), size).write_cells(bg_iter);
        }
        self
    }

    pub fn as_mut_view<'a>(&'a mut self, offset: Point, size: Size) -> BufferMut<'a> {
        BufferMut {
            buffer: self,
            offset,
            size,
        }
    }

    pub fn get_cell(&self, p: Point) -> Option<&Option<Cell>> {
        if p.is_in(self.size()) {
            Some(&self.buffer[p.into_index(self.size())])
        } else {
            None
        }
    }

    pub fn iter<'a>(&'a self, view_size: Size) -> impl Iterator<Item = PointWithCell<'a>> + '_ {
        let buffer_size = self.size;
        self.buffer.iter().enumerate().filter_map(move |(i, cell)| {
            let p = index_into_point(i, buffer_size);
            if p.is_in(view_size) {
                Some(PointWithCell { p, cell })
            } else {
                None
            }
        })
    }

    pub fn get_diff<'a>(
        &'a self,
        old: &'a Buffer,
        view_size: Size,
    ) -> impl Iterator<Item = PointWithCell<'a>> + '_ {
        self.iter(view_size).filter(move |point_with_cell| {
            let old_cell = old.get_cell(point_with_cell.p);

            if let Some(cell) = old_cell {
                cell != point_with_cell.cell
            } else {
                true
            }
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
