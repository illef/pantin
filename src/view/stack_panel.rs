use super::*;

pub struct StackPanel<E: AsUIEvent> {
    children: Vec<Box<dyn View<Event = E>>>,
    bg: Option<color::Color>,
}

pub fn make_stack_panel<E: AsUIEvent>() -> StackPanel<E> {
    StackPanel {
        children: vec![],
        bg: None,
    }
}

impl<E: AsUIEvent> StackPanel<E> {
    pub fn set_bg(mut self, bg: color::Color) -> Self {
        self.bg = Some(bg);
        self
    }

    pub fn clear_children(&mut self) {
        self.children.clear();
    }

    pub fn get_children(&mut self) -> &mut Vec<Box<dyn View<Event = E>>> {
        &mut self.children
    }

    pub fn add_child(mut self, view: Box<dyn View<Event = E>>) -> Self {
        self.children.push(view);
        self
    }

    fn render_child(buffer: &mut BufferMut, children: &mut Vec<Box<dyn View<Event = E>>>) {
        let mut offset = Point(0, 0);
        let mut size = buffer.size();

        for child_view in children.iter_mut() {
            let mut buffer_mut_view = buffer.as_mut_view(offset, size);
            let available_size = available_size(buffer_mut_view.size(), child_view.desire_size());
            if available_size.is_zero() {
                break;
            }
            let mut buffer_mut = buffer_mut_view.as_mut_view(Point(0, 0), available_size);

            child_view.render(&mut buffer_mut);
            offset = offset.add(Point(0, available_size.height));
            size = Size {
                width: buffer_mut_view.size().width,
                height: buffer_mut_view.size().height - available_size.height,
            };
        }
    }
}

impl<E: AsUIEvent> View for StackPanel<E> {
    type Event = E;
    fn desire_size(&self) -> Size {
        let height: u64 = self
            .children
            .iter()
            .map(|view| view.desire_size().height as u64)
            .sum();

        Size {
            width: std::u16::MAX,
            height: height as u16,
        }
    }
    fn render(&mut self, buf: &mut BufferMut) {
        StackPanel::render_child(buf, &mut self.children);
    }
}
