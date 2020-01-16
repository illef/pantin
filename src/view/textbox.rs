use super::utils;
use super::*;

pub struct TextBox<'a, E: AsUIEvent> {
    string: String,
    bg: color::Color,
    fg: color::Color,
    desire_size: Size,
    focused: bool,
    phantom: std::marker::PhantomData<E>,
    text_changed_callback: Vec<Box<dyn FnMut(String) + 'a>>,
}

impl<'a, E: AsUIEvent> TextBox<'a, E> {
    pub fn get_text(&self) -> &String {
        &self.string
    }

    pub fn add_callback<CB: FnMut(String) + 'a>(&mut self, c: CB) {
        self.text_changed_callback.push(Box::new(c));
    }
}

impl<'a, E: AsUIEvent> View for TextBox<'a, E> {
    type Event = E;
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
        let temp = self.string.clone();
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
        let current_str = self.string.clone();
        if temp != current_str {
            self.text_changed_callback
                .iter_mut()
                .for_each(|callback| callback(current_str.clone()));
        }
    }
}

pub fn make_textbox<'a, E: AsUIEvent>(
    desire_size: Size,
    bg: color::Color,
    fg: color::Color,
) -> TextBox<'a, E> {
    TextBox {
        string: String::new(),
        bg,
        fg,
        desire_size,
        focused: true,
        phantom: std::marker::PhantomData,
        text_changed_callback: vec![],
    }
}
