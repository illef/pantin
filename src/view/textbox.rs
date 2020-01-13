use super::utils;
use super::*;

#[derive(Clone)]
pub struct TextBox {
    string: String,
    bg: color::Color,
    fg: color::Color,
    desire_size: Size,
    focused: bool,
}

impl View for TextBox {
    fn desire_size(&self) -> Size {
        self.desire_size
    }
    fn render(&mut self, buf: &mut BufferMut) {
        let iter = utils::str_as_cells(&self.string, self.bg, self.fg);
        let infinite = utils::make_infinite_cells(' ', self.bg, self.fg);
        let cell_iter = iter.chain(infinite);

        buf.write_cells(cell_iter);
    }
}

impl Focusable for TextBox {
    fn is_focused(&self) -> bool {
        self.focused
    }
    fn set_focus(&mut self, focus: bool) {
        self.focused = focus;
    }

    fn handle_key_event(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) => self.string.push(c),
            KeyCode::Backspace => {
                let count = self.string.chars().count();
                if count > 0 {
                    self.string = self.string.chars().take(count - 1).collect();
                }
            }
            KeyCode::Enter => {
                self.string.push('\n');
                let new_line_count = self.string.chars().filter(|c| *c == '\n').count();
                if (new_line_count as u16) < self.desire_size.height {
                    self.desire_size.height += 1;
                }
            }
            _ => {}
        }
    }
}

pub fn make_textbox(desire_size: Size, bg: color::Color, fg: color::Color) -> TextBox {
    TextBox {
        string: String::new(),
        bg,
        fg,
        desire_size,
        focused: true,
    }
}
