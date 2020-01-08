use super::*;

pub struct StackPanel {
    buffer_cache: Option<Buffer>,
    childs: Vec<Box<dyn View>>,
}

pub fn make_stack_panel() -> StackPanel {
    StackPanel {
        buffer_cache: None,
        childs: vec![],
    }
}

impl StackPanel {
    pub fn add_child(mut self, view: Box<dyn View>) -> Self {
        self.childs.push(view);
        self
    }

    fn render_child(mut buffer: Buffer, childs: &mut Vec<Box<dyn View>>) -> Buffer {
        let mut offset = Point(0, 0);
        let mut size = buffer.size();

        for child_view in childs.iter_mut() {
            let mut buffer_mut_view = buffer.as_mut_view(offset, size);
            if buffer_mut_view.size().is_zero() {
                break;
            }
            let height = available_height(buffer_mut_view.size(), child_view.desire_size());
            child_view.render(&mut buffer_mut_view);
            offset = offset.add(Point(0, height));
            size = Size {
                width: buffer_mut_view.size().width,
                height: buffer_mut_view.size().height - height,
            };
        }

        buffer
    }
}

impl View for StackPanel {
    fn desire_size(&self) -> Size {
        let height: u64 = self
            .childs
            .iter()
            .map(|view| view.desire_size().height as u64)
            .sum();

        Size {
            width: std::u16::MAX,
            height: height as u16,
        }
    }
    fn render(&mut self, buf: &mut BufferMutView) {
        if self.buffer_cache.is_none() || self.buffer_cache.as_ref().unwrap().size() != buf.size() {
            self.buffer_cache = Some(StackPanel::render_child(
                Buffer::new(buf.size()),
                &mut self.childs,
            ));
        }
        buf.write_buffer(self.buffer_cache.as_ref().unwrap());
    }
}
