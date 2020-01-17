use super::*;

pub struct ScrollViewer<V: View, E: AsUIEvent> {
    inner_view: V,
    vertical_offset: u16,
    focused: bool,
    phantom: std::marker::PhantomData<E>,
}

impl<V: View, E: AsUIEvent> ScrollViewer<V, E> {
    pub fn set_vertical_offset(&mut self, mut new_offset: u16) -> u16 {
        std::mem::swap(&mut self.vertical_offset, &mut new_offset);
        new_offset
    }

    pub fn get_vertical_offset(&self) -> u16 {
        self.vertical_offset
    }

    pub fn get_inner_view(&mut self) -> &mut V {
        &mut self.inner_view
    }
}

impl<V: View, E: AsUIEvent> View for ScrollViewer<V, E> {
    type Event = E;
    fn is_focusable(&self) -> bool {
        true
    }

    fn is_focused(&self) -> bool {
        self.focused
    }

    fn set_focus(&mut self, focus: bool) {
        self.focused = focus;
    }

    //TODO::key j, key k is hard coded, change it.
    fn handle_key_event(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('j') => {
                self.vertical_offset += 1;
            }
            KeyCode::Char('k') => {
                if self.vertical_offset > 0 {
                    self.vertical_offset -= 1;
                }
            }
            _ => {}
        }
    }
    fn desire_size(&self) -> Size {
        self.inner_view.desire_size()
    }
    fn render(&mut self, buf: &mut BufferMut) {
        let mut cache_buffer = Buffer::new(size(buf.size().width, self.desire_size().height));
        {
            let mut buffer_mut = cache_buffer.as_mut_view(Point(0, 0), cache_buffer.size());
            self.inner_view.render(&mut buffer_mut);
        }
        if buf.size().height >= self.desire_size().height {
            self.vertical_offset = 0;
        } else if buf.size().height >= self.desire_size().height - self.vertical_offset {
            self.vertical_offset = self.desire_size().height - buf.size().height;
        }

        assert!(cache_buffer.size() >= buf.size());
        assert!(cache_buffer.size().height == self.desire_size().height);

        let buffer_span = cache_buffer.as_mut_view(Point(0, self.vertical_offset), buf.size());

        assert_eq!(buffer_span.size(), buf.size());

        for y in 0..buffer_span.size().height {
            for x in 0..buffer_span.size().width {
                if let Some(cell) = buffer_span.get_cell(Point(x, y)) {
                    buf.write_cell(Point(x, y), *cell);
                }
            }
        }
    }
}

pub fn make_scroll_viewer<V: View, E: AsUIEvent>(v: V) -> ScrollViewer<V, E> {
    ScrollViewer {
        inner_view: v,
        vertical_offset: 0,
        focused: true,
        phantom: std::marker::PhantomData,
    }
}
