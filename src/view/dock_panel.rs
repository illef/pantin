use super::utils;
use super::*;
use std::sync::Arc;

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

    fn render_child(&mut self) {
        let buffer = self.buffer_cache.as_mut().unwrap();
        let mut offset = Point(0, 0);
        let mut size = buffer.size();

        for child in self.childs.iter_mut() {
            if child.0 == Alignment::Left {
                let mut buffer_mut_view = self
                    .buffer_cache
                    .as_mut()
                    .unwrap()
                    .as_mut_view(offset, size);
                let width = child.1.desire_size().width;

                let mut child_mut_view = buffer_mut_view.as_mut_view(
                    offset,
                    Size {
                        width,
                        height: buffer_mut_view.size().height,
                    },
                );
                child.1.render(&mut child_mut_view);
                offset = offset.add(Point(width, 0));
                size = Size {
                    width: size.width - width,
                    height: size.height,
                }
            } else if child.0 == Alignment::Bottom {
                let mut buffer_mut_view = self
                    .buffer_cache
                    .as_mut()
                    .unwrap()
                    .as_mut_view(offset, size);
                let height = child.1.desire_size().height;

                let mut child_mut_view = buffer_mut_view.as_mut_view(
                    Point(0, size.height - height),
                    Size {
                        width: size.width,
                        height: height,
                    },
                );
                child.1.render(&mut child_mut_view);
                size = Size {
                    width: size.width,
                    height: size.height - height,
                }
            } else if child.0 == Alignment::Top {
                let mut buffer_mut_view = self
                    .buffer_cache
                    .as_mut()
                    .unwrap()
                    .as_mut_view(offset, size);
                let height = child.1.desire_size().height;

                let mut child_mut_view = buffer_mut_view.as_mut_view(
                    Point(0, 0),
                    Size {
                        width: size.width,
                        height: height,
                    },
                );
                child.1.render(&mut child_mut_view);
                offset = offset.add(Point(0, height));
                size = Size {
                    width: size.width,
                    height: size.height - height,
                }
            }
        }
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
            self.buffer_cache = Some(Buffer::new(buf.size()));
            self.render_child();
        }
        buf.write_buffer(self.buffer_cache.as_ref().unwrap());
    }
}
