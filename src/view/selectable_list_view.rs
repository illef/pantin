use super::*;

pub trait AsSelectedListViewItem<E: AsUIEvent>: Clone + PartialEq {
    fn as_not_selected_view(&self) -> Box<dyn View<Event = E>>;
    fn as_selected_view(&self) -> Box<dyn View<Event = E>>;
}

pub struct SelectableListView<'a, E: AsUIEvent, S: AsSelectedListViewItem<E>> {
    items: Vec<S>,
    scroll_viewer: ScrollViewer<StackPanel<E>, E>,
    is_focused: bool,
    selected_info: Option<(usize, S)>,
    selected_item_changed_callback: Vec<Box<dyn FnMut(Option<(usize, S)>) + 'a>>,
}

fn calc_offset_delta<'a, E: AsUIEvent + 'static>(
    available_size: Size,
    mut vertical_offset: i32,
    index: usize,
    items: impl Iterator<Item = &'a Box<dyn View<Event = E>>>,
) -> i16 {
    let mut available_height = available_size.height as i32;

    for (i, view) in items.enumerate() {
        let desire_height = view.desire_size().height as i32;

        if i == index {
            if vertical_offset > 0 {
                return (vertical_offset * -1) as i16;
            } else if available_height < desire_height {
                return (desire_height - available_height - vertical_offset) as i16;
            }
        }

        if vertical_offset > 0 {
            //case of skip
            vertical_offset -= desire_height;
        } else {
            available_height -= desire_height;
        }
    }

    0
}

impl<'a, E: AsUIEvent, S: AsSelectedListViewItem<E>> SelectableListView<'a, E, S> {
    pub fn add_callback<CB: FnMut(Option<(usize, S)>) + 'a>(&mut self, c: CB) {
        self.selected_item_changed_callback.push(Box::new(c));
        let selected_info = self.selected_info.clone();
        self.selected_item_changed_callback.last_mut().unwrap()(selected_info);
    }
    pub fn set_items(&mut self, items: Vec<S>) {
        self.items = items;

        self.scroll_viewer.get_inner_view().clear_children();

        if self.items.len() == 0 {
            self.set_selected_index(None);
            return;
        }

        let mut new_index = 0;
        if let Some(last_selected_item) = self.selected_info.as_ref() {
            if let Some((i, _)) = self
                .items
                .iter()
                .enumerate()
                .filter(|item| *item.1 == last_selected_item.1)
                .next()
            {
                new_index = i;
            }
        }

        for (i, item) in self.items.iter().enumerate() {
            let stack_panel = self.scroll_viewer.get_inner_view();
            if i == new_index {
                stack_panel.add_child(item.as_selected_view());
            } else {
                stack_panel.add_child(item.as_not_selected_view());
            }
        }

        self.set_selected_index(Some(new_index));
    }

    fn set_selected_index(&mut self, new_index: Option<usize>) {
        if new_index == None {
            self.selected_item_changed_callback
                .iter_mut()
                .for_each(|callback| callback(None));
            return;
        }

        let new_index = new_index.unwrap();
        assert!(new_index < self.items.len());
        let new_selected_item = self.items[new_index].clone();

        let mut last_selected_info = Some((new_index, new_selected_item));
        std::mem::swap(&mut last_selected_info, &mut self.selected_info);

        if last_selected_info != None {
            let (last_selected_index, last_selected_item) = last_selected_info.unwrap();

            let last_selected_view = last_selected_item.as_not_selected_view();
            let selected_view = self.items[new_index].as_selected_view();

            self.scroll_viewer
                .get_inner_view()
                .swap_child(last_selected_index, last_selected_view);

            self.scroll_viewer
                .get_inner_view()
                .swap_child(new_index, selected_view);

            let selected_info = self.selected_info.clone();

            self.selected_item_changed_callback
                .iter_mut()
                .for_each(|callback| callback(selected_info.clone()));
        }
    }
}

impl<'a, E: AsUIEvent + 'static, S: AsSelectedListViewItem<E>> View
    for SelectableListView<'a, E, S>
{
    type Event = E;
    fn desire_size(&self) -> Size {
        self.scroll_viewer.desire_size()
    }
    fn render(&mut self, buf: &mut BufferMut) {
        if self.selected_info == None {
            return;
        }
        let offset = calc_offset_delta(
            buf.size(),
            self.scroll_viewer.get_vertical_offset() as i32,
            self.selected_info.as_ref().unwrap().0,
            self.scroll_viewer.get_inner_view().get_children().iter(),
        ) + self.scroll_viewer.get_vertical_offset() as i16;

        self.scroll_viewer.set_vertical_offset(offset as u16);
        self.scroll_viewer.render(buf)
    }

    fn is_focusable(&self) -> bool {
        true
    }

    fn is_focused(&self) -> bool {
        self.is_focused
    }

    fn set_focus(&mut self, focus: bool) {
        self.is_focused = focus
    }

    //TODO::key j, key k is hard coded, change it.
    fn handle_key_event(&mut self, key: KeyCode) {
        if self.items.len() == 0 {
            return;
        }
        assert!(self.selected_info.is_some());
        let selected_index = self.selected_info.as_ref().unwrap().0;

        match key {
            KeyCode::Char('j') => {
                if selected_index + 1 < self.items.len() {
                    self.set_selected_index(Some(selected_index + 1));
                }
            }
            KeyCode::Char('k') => {
                if selected_index > 0 {
                    self.set_selected_index(Some(selected_index - 1));
                }
            }
            _ => {}
        }
    }
}

pub fn make_selectable_list_view<'a, E: AsUIEvent + 'static, S: AsSelectedListViewItem<E>>(
    items: Vec<S>,
) -> SelectableListView<'a, E, S> {
    let mut view = SelectableListView {
        items: vec![],
        scroll_viewer: make_scroll_viewer(make_stack_panel()),
        selected_info: None,
        is_focused: true,
        selected_item_changed_callback: vec![],
    };

    view.set_items(items);
    view
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::Event;
    use crossterm::event::KeyEvent;

    #[derive(Debug, Clone)]
    pub enum BasicEvent {
        Key(KeyEvent),
        Resize(Size),
        Other,
    }

    impl AsUIEvent for BasicEvent {
        fn as_ui_event(&self) -> Option<Event> {
            match self {
                BasicEvent::Key(key_event) => Some(Event::Key(*key_event)),
                BasicEvent::Resize(size) => Some(Event::Resize(size.width, size.height)),
                _ => None,
            }
        }
        fn from_tui_event(e: Event) -> Self {
            match e {
                Event::Key(key_event) => BasicEvent::Key(key_event),
                Event::Resize(w, h) => BasicEvent::Resize(size(w, h)),
                _ => BasicEvent::Other,
            }
        }
    }

    fn make_test_view_items(len: usize, height: u16) -> Vec<Box<dyn View<Event = BasicEvent>>> {
        let mut vec: Vec<Box<dyn View<Event = BasicEvent>>> = vec![];

        for _ in 0..len {
            vec.push(Box::new(make_fill(color::Color::Reset, size(MAX, height))));
        }
        vec
    }

    #[test]
    fn calc_visible_range_test_1() {
        let v = make_test_view_items(0, 1);
        let result = calc_offset_delta(size(MAX, 0), 0, 0, v.iter());
        assert_eq!(result, 0);
    }

    #[test]
    fn calc_visible_range_test_2() {
        let v = make_test_view_items(3, 1);
        let result = calc_offset_delta(size(MAX, 1), 0, 0, v.iter());
        assert_eq!(result, 0);

        let result = calc_offset_delta(size(MAX, 1), 1, 0, v.iter());
        assert_eq!(result, -1);

        let result = calc_offset_delta(size(MAX, 1), 2, 1, v.iter());
        assert_eq!(result, -1);

        let result = calc_offset_delta(size(MAX, 1), 2, 0, v.iter());
        assert_eq!(result, -2);
    }

    #[test]
    fn calc_visible_range_test_3() {
        let v = make_test_view_items(4, 1);

        let result = calc_offset_delta(size(MAX, 1), 0, 1, v.iter());
        assert_eq!(result, 1);

        let result = calc_offset_delta(size(MAX, 1), 0, 2, v.iter());
        assert_eq!(result, 2);

        let result = calc_offset_delta(size(MAX, 1), 1, 2, v.iter());
        assert_eq!(result, 1);
    }

    #[test]
    fn calc_visible_range_test_4() {
        {
            let v = make_test_view_items(2, 5);

            let result = calc_offset_delta(size(MAX, 6), 0, 1, v.iter());
            assert_eq!(result, 4);
        }
        {
            let v = make_test_view_items(5, 2);
            let result = calc_offset_delta(size(MAX, 5), 1, 3, v.iter());
            assert_eq!(result, 2);
        }
    }
}
