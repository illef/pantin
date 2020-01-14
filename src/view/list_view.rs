use super::*;

pub struct ListView<S, E: AsUIEvent> {
    stack_panel: StackPanel<E>,
    phantom: std::marker::PhantomData<S>,
}

impl<S, E: AsUIEvent> ListView<S, E> {
    pub fn set_bg(mut self, bg: color::Color) -> Self {
        self.stack_panel = self.stack_panel.set_bg(bg);
        self
    }
}

impl<S, E: AsUIEvent> View for ListView<S, E> {
    type Event = E;
    fn desire_size(&self) -> Size {
        self.stack_panel.desire_size()
    }
    fn render(&mut self, buf: &mut BufferMut) {
        self.stack_panel.render(buf)
    }
}

pub fn make_list_view<E: AsUIEvent, S: Into<Box<dyn View<Event = E>>>>(
    iter: impl Iterator<Item = S>,
) -> ListView<S, E> {
    let mut stack_panel = make_stack_panel();
    for s in iter {
        stack_panel = stack_panel.add_child(s.into());
    }
    ListView {
        stack_panel,
        phantom: std::marker::PhantomData,
    }
}
