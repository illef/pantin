use std::fmt;
use termion::color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    White,
    Rgb(u8, u8, u8),
}

pub struct Fg(pub Color);

pub struct Bg(pub Color);

impl fmt::Display for Fg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Color::Reset => write!(f, "{}", color::Fg(color::Reset {})),
            Color::Black => write!(f, "{}", color::Fg(color::Black {})),
            Color::Red => write!(f, "{}", color::Fg(color::Red {})),
            Color::Green => write!(f, "{}", color::Fg(color::Green {})),
            Color::Yellow => write!(f, "{}", color::Fg(color::Yellow {})),
            Color::Blue => write!(f, "{}", color::Fg(color::Blue {})),
            Color::Magenta => write!(f, "{}", color::Fg(color::Magenta {})),
            Color::Cyan => write!(f, "{}", color::Fg(color::Cyan {})),
            Color::LightRed => write!(f, "{}", color::Fg(color::LightRed {})),
            Color::LightGreen => write!(f, "{}", color::Fg(color::LightGreen {})),
            Color::LightYellow => write!(f, "{}", color::Fg(color::LightYellow {})),
            Color::LightBlue => write!(f, "{}", color::Fg(color::LightBlue {})),
            Color::LightMagenta => write!(f, "{}", color::Fg(color::LightMagenta {})),
            Color::LightCyan => write!(f, "{}", color::Fg(color::LightCyan {})),
            Color::White => write!(f, "{}", color::Fg(color::White {})),
            Color::Rgb(r, g, b) => write!(f, "{}", color::Fg(color::Rgb(r, g, b))),
        }
    }
}

impl fmt::Display for Bg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Color::Reset => write!(f, "{}", color::Bg(color::Reset {})),
            Color::Black => write!(f, "{}", color::Bg(color::Black {})),
            Color::Red => write!(f, "{}", color::Bg(color::Red {})),
            Color::Green => write!(f, "{}", color::Bg(color::Green {})),
            Color::Yellow => write!(f, "{}", color::Bg(color::Yellow {})),
            Color::Blue => write!(f, "{}", color::Bg(color::Blue {})),
            Color::Magenta => write!(f, "{}", color::Bg(color::Magenta {})),
            Color::Cyan => write!(f, "{}", color::Bg(color::Cyan {})),
            Color::LightRed => write!(f, "{}", color::Bg(color::LightRed {})),
            Color::LightGreen => write!(f, "{}", color::Bg(color::LightGreen {})),
            Color::LightYellow => write!(f, "{}", color::Bg(color::LightYellow {})),
            Color::LightBlue => write!(f, "{}", color::Bg(color::LightBlue {})),
            Color::LightMagenta => write!(f, "{}", color::Bg(color::LightMagenta {})),
            Color::LightCyan => write!(f, "{}", color::Bg(color::LightCyan {})),
            Color::White => write!(f, "{}", color::Bg(color::White {})),
            Color::Rgb(r, g, b) => write!(f, "{}", color::Bg(color::Rgb(r, g, b))),
        }
    }
}
