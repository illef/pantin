use super::buffer;
use super::*;

struct InfiniteCells {
    ch: char,
    bg: color::Color,
    fg: color::Color,
}

struct Cursor {
    called: bool,
    bg: color::Color,
    fg: color::Color,
}

impl Iterator for Cursor {
    type Item = Cell;

    fn next(&mut self) -> Option<Cell> {
        if self.called == false {
            self.called = true;
            Some(Cell {
                ch: ' ',
                bg: self.bg,
                fg: self.fg,
                cursor_on: true,
            })
        } else {
            None
        }
    }
}

impl Iterator for InfiniteCells {
    type Item = Cell;

    fn next(&mut self) -> Option<Cell> {
        Some(Cell {
            ch: self.ch,
            bg: self.bg,
            fg: self.fg,
            cursor_on: false,
        })
    }
}

pub fn chars_into_cells(
    chars: impl Iterator<Item = char>,
    bg: color::Color,
    fg: color::Color,
) -> impl Iterator<Item = Cell> {
    chars.map(move |ch| Cell {
        ch,
        bg,
        fg,
        cursor_on: false,
    })
}

pub fn str_as_cells<S: AsRef<str>>(
    s: S,
    bg: color::Color,
    fg: color::Color,
) -> impl Iterator<Item = Cell> {
    let vec: Vec<char> = s.as_ref().chars().collect();
    chars_into_cells(vec.into_iter(), bg, fg)
}

pub fn make_cursor_cell(bg: color::Color, fg: color::Color) -> impl Iterator<Item = Cell> {
    Cursor {
        bg,
        fg,
        called: false,
    }
}

pub fn make_infinite_cells(
    ch: char,
    bg: color::Color,
    fg: color::Color,
) -> impl Iterator<Item = Cell> {
    InfiniteCells { ch, bg, fg }
}
