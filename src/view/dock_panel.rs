use super::utils;
use super::*;
use std::sync::Arc;

pub struct DockPanel {
    buffer_cache: Option<Buffer>,
    childs: Vec<(Alignment, Arc<dyn View>)>,
}

pub fn make_dock_panel() -> DockPanel {
    DockPanel {
        buffer_cache: None,
        childs: vec![],
    }
}

impl DockPanel {
    pub fn add_child(&mut self, alignment: Alignment, view: Arc<dyn View>) {
        self.childs.push((alignment, view))
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
        }
        buf.write_buffer(self.buffer_cache.as_ref().unwrap());
    }
}
