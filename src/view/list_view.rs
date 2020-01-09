use super::utils;
use super::*;

pub struct ListView<S> {
    stack_panel: StackPanel,
    phantom: std::marker::PhantomData<S>,
}

impl<S> ListView<S> {
    pub fn set_bg(mut self, bg: color::Color) -> Self {
        self.stack_panel = self.stack_panel.set_bg(bg);
        self
    }
}

impl<S> View for ListView<S> {
    fn desire_size(&self) -> Size {
        self.stack_panel.desire_size()
    }
    fn render(&mut self, buf: &mut BufferMut) {
        self.stack_panel.render(buf)
    }
}

pub fn make_list_view<S: Into<Box<dyn View>>>(iter: impl Iterator<Item = S>) -> ListView<S> {
    let mut stack_panel = make_stack_panel();
    for s in iter {
        stack_panel = stack_panel.add_child(s.into());
    }
    ListView {
        stack_panel,
        phantom: std::marker::PhantomData,
    }
}
