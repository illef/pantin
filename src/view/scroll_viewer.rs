use super::*;

pub struct ScrollViewer<V: View, E: AsUIEvent> {
    inner_view: V,
    vertical_offset: u16,
    focused: bool,
    desire_size: Size,
    cache_buffer: Option<Buffer>,
    phantom: std::marker::PhantomData<E>,
}

impl<V: View, E: AsUIEvent> ScrollViewer<V, E> {
    pub fn set_vertical_offset(&mut self, mut new_offset: u16) -> u16 {
        std::mem::swap(&mut self.vertical_offset, &mut new_offset);
        new_offset
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
        self.desire_size
    }
    fn render(&mut self, buf: &mut BufferMut) {
        if self.cache_buffer.is_none()
            || self.cache_buffer.as_ref().unwrap().size().width < buf.size().width
        {
            let desize_height = self.desire_size.height;
            let mut buffer_cache = Buffer::new(size(buf.size().width, desize_height));
            {
                let mut buffer_mut = buffer_cache.as_mut_view(Point(0, 0), buffer_cache.size());
                self.inner_view.render(&mut buffer_mut);
            }
            self.cache_buffer = Some(buffer_cache);
        }
        if buf.size().height >= self.desire_size().height {
            self.vertical_offset = 0;
        } else if buf.size().height >= self.desire_size().height - self.vertical_offset {
            self.vertical_offset = self.desire_size().height - buf.size().height;
        }

        assert!(self.cache_buffer.is_some());
        assert!(self.cache_buffer.as_ref().unwrap().size() >= buf.size());
        assert!(self.cache_buffer.as_ref().unwrap().size().height == self.desire_size.height);

        let buffer_span = self
            .cache_buffer
            .as_mut()
            .unwrap()
            .as_mut_view(Point(0, self.vertical_offset), buf.size());

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
        cache_buffer: None,
        desire_size: v.desire_size(),
        inner_view: v,
        vertical_offset: 0,
        focused: true,
        phantom: std::marker::PhantomData,
    }
}
