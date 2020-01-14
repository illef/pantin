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

impl TextBox {
    pub fn get_text(&self) -> &String {
        &self.string
    }
}

impl View for TextBox {
    fn desire_size(&self) -> Size {
        self.desire_size
    }
    fn render(&mut self, buf: &mut BufferMut) {
        let iter = utils::str_as_cells(&self.string, self.bg, self.fg)
            .chain(utils::make_cursor_cell(self.bg, self.fg))
            .chain(utils::make_infinite_cells(' ', self.bg, self.fg));

        buf.write_cells(iter);
    }

    fn is_focusable(&self) -> bool {
        true
    }

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
