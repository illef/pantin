pub mod backend;
pub mod buffer;
pub mod color;
pub mod error;
pub mod utils;
pub mod view;

use color::*;

#[derive(Clone, PartialEq, Copy)]
pub struct Cell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
}

#[derive(Clone, PartialEq, Copy)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[derive(Clone, PartialEq, Copy)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct PointWithCell<'a> {
    pub p: Point,
    pub cell: Option<&'a Cell>,
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
