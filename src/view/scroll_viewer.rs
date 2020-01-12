use super::*;

pub struct ScrollViewer<V: View> {
    inner_view: V,
    vertical_offset: u16,
}

impl<V: View> ScrollViewer<V> {
    fn set_vertical_offset(&mut self, mut new_offset: u16) -> u16 {
        std::mem::swap(&mut self.vertical_offset, &mut new_offset);
        new_offset
    }
}

impl<V: View> View for ScrollViewer<V> {
    fn desire_size(&self) -> Size {
        self.inner_view.desire_size()
    }
    fn render(&mut self, buf: &mut BufferMut) {
        let size = buf.size() + size(0, self.vertical_offset);
        if size == buf.size() {
            self.inner_view.render(buf);
        } else {
            //TODO::case of vertical_offset is greater than desire_size().height
            let mut temp_buffer = Buffer::new(size);
            {
                let mut buffer_mut = temp_buffer.as_mut_view(Point(0, 0), size);
                self.inner_view.render(&mut buffer_mut);
            }
            let buffer_span = temp_buffer.as_mut_view(
                Point(0, self.vertical_offset),
                size - Point(0, self.vertical_offset),
            );

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
}

pub fn make_scroll_viewer<V: View>(v: V) -> ScrollViewer<V> {
    ScrollViewer {
        inner_view: v,
        vertical_offset: 0,
    }
}
