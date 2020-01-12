pub mod buffer;
pub mod color;
pub mod error;
pub mod utils;
pub mod view;

use std::ops::{Add, Sub};

pub use color::*;
pub use view::View;

#[derive(Clone, PartialEq, Copy)]
pub struct Cell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
}

pub const MAX: u16 = std::u16::MAX;

#[derive(Clone, PartialEq, Copy, Debug)]
pub struct Point(pub u16, pub u16);

impl Point {
    pub fn is_in(&self, size: Size) -> bool {
        self.0 < size.width && self.1 < size.height
    }

    pub fn into_index(&self, size: Size) -> usize {
        (self.0 as usize) + (self.1 as usize * size.width as usize)
    }

    pub fn into_goto(&self) -> crossterm::cursor::MoveTo {
        crossterm::cursor::MoveTo(self.0, self.1)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, p: Point) -> Self::Output {
        Point(self.0 + p.0, self.1 + p.1)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, p: Point) -> Self::Output {
        Point(self.0 - p.0, self.1 - p.1)
    }
}
pub fn index_into_point(i: usize, size: Size) -> Point {
    Point(i as u16 % size.width, i as u16 / size.width)
}

#[derive(Clone, PartialEq, Copy, Debug)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub fn size(width: u16, height: u16) -> Size {
    Size { width, height }
}

impl Size {
    pub fn is_zero(&self) -> bool {
        self.width == 0 || self.height == 0
    }
}

pub fn terminal_size() -> Size {
    let (width, height) = crossterm::terminal::size().unwrap();
    size(width, height)
}

pub struct PointWithCell<'a> {
    pub p: Point,
    pub cell: &'a Option<Cell>,
}

pub struct PointWithMutCell<'a> {
    pub p: Point,
    pub cell: &'a mut Option<Cell>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
