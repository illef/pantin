use super::*;

pub struct DockPanel {
    buffer_cache: Option<Buffer>,
    childs: Vec<(Alignment, Box<dyn View>)>,
}

pub fn make_dock_panel() -> DockPanel {
    DockPanel {
        buffer_cache: None,
        childs: vec![],
    }
}

impl DockPanel {
    pub fn add_child(&mut self, alignment: Alignment, view: Box<dyn View>) {
        self.childs.push((alignment, view))
    }

    fn render_nothing(buffer_mut_view: BufferMutView, _: &mut Box<dyn View>) -> (Point, Size) {
        (Point(0, 0), buffer_mut_view.size())
    }

    fn render_right(
        mut buffer_mut_view: BufferMutView,
        child_view: &mut Box<dyn View>,
    ) -> (Point, Size) {
        let width = child_view.desire_size().width;

        let mut child_mut_view = buffer_mut_view.as_mut_view(
            Point(buffer_mut_view.size().width - width, 0),
            Size {
                width: buffer_mut_view.size().width - width,
                height: buffer_mut_view.size().height,
            },
        );
        child_view.render(&mut child_mut_view);
        let offset = Point(0, 0);
        let size = Size {
            width: buffer_mut_view.size().width - width,
            height: buffer_mut_view.size().height,
        };

        (offset, size)
    }

    fn render_left(
        mut buffer_mut_view: BufferMutView,
        child_view: &mut Box<dyn View>,
    ) -> (Point, Size) {
        let width = child_view.desire_size().width;

        let mut child_mut_view = buffer_mut_view.as_mut_view(
            Point(0, 0),
            Size {
                width,
                height: buffer_mut_view.size().height,
            },
        );
        child_view.render(&mut child_mut_view);
        let offset = Point(width, 0);
        let size = Size {
            width: buffer_mut_view.size().width - width,
            height: buffer_mut_view.size().height,
        };

        (offset, size)
    }

    fn render_bottom(
        mut buffer_mut_view: BufferMutView,
        child_view: &mut Box<dyn View>,
    ) -> (Point, Size) {
        let height = child_view.desire_size().height;
        let mut child_mut_view = buffer_mut_view.as_mut_view(
            Point(0, buffer_mut_view.size().height - height),
            Size {
                width: buffer_mut_view.size().width,
                height: height,
            },
        );
        child_view.render(&mut child_mut_view);
        let size = Size {
            width: buffer_mut_view.size().width,
            height: buffer_mut_view.size().height - height,
        };
        (Point(0, 0), size)
    }

    fn render_top(
        mut buffer_mut_view: BufferMutView,
        child_view: &mut Box<dyn View>,
    ) -> (Point, Size) {
        let height = child_view.desire_size().height;
        let mut child_mut_view = buffer_mut_view.as_mut_view(
            Point(0, 0),
            Size {
                width: buffer_mut_view.size().width,
                height: height,
            },
        );
        child_view.render(&mut child_mut_view);
        let size = Size {
            width: buffer_mut_view.size().width,
            height: buffer_mut_view.size().height - height,
        };
        (Point(0, height), size)
    }

    fn render_child(mut buffer: Buffer, childs: &mut Vec<(Alignment, Box<dyn View>)>) -> Buffer {
        let mut offset = Point(0, 0);
        let mut size = buffer.size();

        for child in childs.iter_mut() {
            let buffer_mut_view = buffer.as_mut_view(offset, size);
            let func = match child.0 {
                Alignment::Left => Self::render_left,
                Alignment::Bottom => Self::render_bottom,
                Alignment::Top => Self::render_top,
                Alignment::Right => Self::render_right,
                _ => Self::render_nothing,
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
        Size {
            width: std::u16::MAX,
            height: 1,
        }
    }
    fn render(&mut self, buf: &mut BufferMutView) {
        if self.buffer_cache.is_none() || self.buffer_cache.as_ref().unwrap().size() != buf.size() {
            self.buffer_cache = Some(DockPanel::render_child(
                Buffer::new(buf.size()),
                &mut self.childs,
            ));
        }
        buf.write_buffer(self.buffer_cache.as_ref().unwrap());
    }
}
