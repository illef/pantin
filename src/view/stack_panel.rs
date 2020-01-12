use super::*;

pub struct StackPanel {
    childs: Vec<Box<dyn View>>,
    bg: Option<color::Color>,
}

pub fn make_stack_panel() -> StackPanel {
    StackPanel {
        childs: vec![],
        bg: None,
    }
}

impl StackPanel {
    pub fn set_bg(mut self, bg: color::Color) -> Self {
        self.bg = Some(bg);
        self
    }
    pub fn add_child(mut self, view: Box<dyn View>) -> Self {
        self.childs.push(view);
        self
    }

    fn render_child(buffer: &mut BufferMut, childs: &mut Vec<Box<dyn View>>) {
        let mut offset = Point(0, 0);
        let mut size = buffer.size();

        for child_view in childs.iter_mut() {
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
    fn render(&mut self, buf: &mut BufferMut) {
        StackPanel::render_child(buf, &mut self.childs);
    }
}
