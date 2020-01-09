use super::*;

#[derive(Copy, PartialEq, Clone)]
pub enum Dock {
    Left,
    Right,
    Top,
    Bottom,
    Full,
}

pub struct DockPanel {
    desire_size: Size,
    buffer_cache: Option<Buffer>,
    childs: Vec<(Dock, Box<dyn View>)>,
    bg: Option<color::Color>,
}

pub fn make_dock_panel(size: Size) -> DockPanel {
    DockPanel {
        buffer_cache: None,
        childs: vec![],
        desire_size: size,
        bg: None,
    }
}

impl DockPanel {
    pub fn set_bg(mut self, bg: color::Color) -> Self {
        self.bg = Some(bg);
        self
    }
    pub fn add_child(mut self, dock: Dock, view: Box<dyn View>) -> Self {
        self.childs.push((dock, view));
        self
    }

    fn render_right(
        mut buffer_mut_view: BufferMut,
        child_view: &mut Box<dyn View>,
    ) -> (Point, Size) {
        let width = available_width(buffer_mut_view.size(), child_view.desire_size());

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
        child_view: &mut Box<dyn View>,
    ) -> (Point, Size) {
        let width = available_width(buffer_mut_view.size(), child_view.desire_size());

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
        child_view: &mut Box<dyn View>,
    ) -> (Point, Size) {
        let height = available_height(buffer_mut_view.size(), child_view.desire_size());
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

    fn render_top(mut buffer_mut_view: BufferMut, child_view: &mut Box<dyn View>) -> (Point, Size) {
        let height = available_height(buffer_mut_view.size(), child_view.desire_size());
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
        child_view: &mut Box<dyn View>,
    ) -> (Point, Size) {
        child_view.render(&mut buffer_mut_view);
        (Point(0, 0), size(0, 0))
    }

    fn render_child(mut buffer: Buffer, childs: &mut Vec<(Dock, Box<dyn View>)>) -> Buffer {
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

        buffer
    }
}

impl View for DockPanel {
    fn desire_size(&self) -> Size {
        self.desire_size
    }
    fn render(&mut self, buf: &mut BufferMut) {
        if self.buffer_cache.is_none() || self.buffer_cache.as_ref().unwrap().size() != buf.size() {
            self.buffer_cache = Some(DockPanel::render_child(
                Buffer::new(buf.size()).set_bg(self.bg),
                &mut self.childs,
            ));
        }
        buf.write_buffer(self.buffer_cache.as_ref().unwrap());
    }
}
