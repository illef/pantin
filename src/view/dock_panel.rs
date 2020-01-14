use super::*;

#[derive(Copy, PartialEq, Clone)]
pub enum Dock {
    Left,
    Right,
    Top,
    Bottom,
    Full,
}

pub struct DockPanel<E: AsUIEvent> {
    desire_size: Size,
    childs: Vec<(Dock, Box<dyn View<Event = E>>)>,
    bg: Option<color::Color>,
}

pub fn make_dock_panel<Event: AsUIEvent>(size: Size) -> DockPanel<Event> {
    DockPanel {
        childs: vec![],
        desire_size: size,
        bg: None,
    }
}

impl<E: AsUIEvent> DockPanel<E> {
    pub fn set_bg(mut self, bg: color::Color) -> Self {
        self.bg = Some(bg);
        self
    }
    pub fn add_child(mut self, dock: Dock, view: Box<dyn View<Event = E>>) -> Self {
        self.childs.push((dock, view));
        self
    }

    fn render_right(
        mut buffer_mut_view: BufferMut,
        child_view: &mut Box<dyn View<Event = E>>,
    ) -> (Point, Size) {
        let width = available_size(buffer_mut_view.size(), child_view.desire_size()).width;

        let mut child_mut_view = buffer_mut_view.as_mut_view(
            Point(buffer_mut_view.size().width - width, 0),
            size(width, buffer_mut_view.size().height),
        );
        child_view.render(&mut child_mut_view);
        let offset = Point(0, 0);
        let size = size(
            buffer_mut_view.size().width - width,
            buffer_mut_view.size().height,
        );

        (offset, size)
    }

    fn render_left(
        mut buffer_mut_view: BufferMut,
        child_view: &mut Box<dyn View<Event = E>>,
    ) -> (Point, Size) {
        let width = available_size(buffer_mut_view.size(), child_view.desire_size()).width;

        let mut child_mut_view =
            buffer_mut_view.as_mut_view(Point(0, 0), size(width, buffer_mut_view.size().height));
        child_view.render(&mut child_mut_view);
        let offset = Point(width, 0);

        (
            offset,
            size(
                buffer_mut_view.size().width - width,
                buffer_mut_view.size().height,
            ),
        )
    }

    fn render_bottom(
        mut buffer_mut_view: BufferMut,
        child_view: &mut Box<dyn View<Event = E>>,
    ) -> (Point, Size) {
        let height = available_size(buffer_mut_view.size(), child_view.desire_size()).height;
        let mut child_mut_view = buffer_mut_view.as_mut_view(
            Point(0, buffer_mut_view.size().height - height),
            size(buffer_mut_view.size().width, height),
        );
        child_view.render(&mut child_mut_view);
        (
            Point(0, 0),
            size(
                buffer_mut_view.size().width,
                buffer_mut_view.size().height - height,
            ),
        )
    }

    fn render_top(
        mut buffer_mut_view: BufferMut,
        child_view: &mut Box<dyn View<Event = E>>,
    ) -> (Point, Size) {
        let height = available_size(buffer_mut_view.size(), child_view.desire_size()).height;
        let mut child_mut_view = buffer_mut_view.as_mut_view(
            Point(0, 0),
            Size {
                width: buffer_mut_view.size().width,
                height: height,
            },
        );
        child_view.render(&mut child_mut_view);
        (
            Point(0, height),
            size(
                buffer_mut_view.size().width,
                buffer_mut_view.size().height - height,
            ),
        )
    }

    fn render_full(
        mut buffer_mut_view: BufferMut,
        child_view: &mut Box<dyn View<Event = E>>,
    ) -> (Point, Size) {
        child_view.render(&mut buffer_mut_view);
        (Point(0, 0), size(0, 0))
    }

    fn render_child(buffer: &mut BufferMut, childs: &mut Vec<(Dock, Box<dyn View<Event = E>>)>) {
        let mut offset = Point(0, 0);
        let mut size = buffer.size();

        for child in childs.iter_mut() {
            let buffer_mut_view = buffer.as_mut_view(offset, size);
            if buffer_mut_view.size().is_zero() {
                break;
            }
            let func = match child.0 {
                Dock::Left => Self::render_left,
                Dock::Bottom => Self::render_bottom,
                Dock::Top => Self::render_top,
                Dock::Right => Self::render_right,
                Dock::Full => Self::render_full,
            };

            let (offset_, size_) = func(buffer_mut_view, &mut child.1);
            offset = offset.add(offset_);
            size = size_;
        }
    }
}

impl<E: AsUIEvent> View for DockPanel<E> {
    type Event = E;
    fn desire_size(&self) -> Size {
        self.desire_size
    }
    fn render(&mut self, buf: &mut BufferMut) {
        DockPanel::render_child(buf, &mut self.childs)
    }

    //for focusable
    fn is_focusable(&self) -> bool {
        self.childs.iter().any(|child| child.1.is_focusable())
    }
    fn is_focused(&self) -> bool {
        self.childs.iter().any(|child| child.1.is_focused())
    }
    fn set_focus(&mut self, focus: bool) {
        if let Some(child) = self
            .childs
            .iter_mut()
            .filter(|child| child.1.is_focused())
            .next()
        {
            child.1.set_focus(focus);
        }
    }
    fn handle_key_event(&mut self, key: KeyCode) {
        if let Some(child) = self
            .childs
            .iter_mut()
            .filter(|child| child.1.is_focused())
            .next()
        {
            child.1.handle_key_event(key);
        }
    }
}
