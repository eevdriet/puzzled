use ratatui::{
    prelude::Rect,
    text::Text,
    widgets::{List, ListItem, ListState, StatefulWidgetRef, Widget},
};

pub struct ListWidget<T> {
    items: Vec<T>,
    state: ListState,
}

impl<T> ListWidget<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {
            items,
            state: ListState::default(),
        }
    }
}

impl<'a, T> StatefulWidgetRef for ListWidget<T>
where
    T: Into<ListItem<'a>>,
{
    type State = ListState;

    fn render_ref(&self, area: Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {}
}
