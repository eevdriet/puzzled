use std::marker::PhantomData;

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect, Size},
    style::{Color, Style},
    widgets::{Block, BorderType, List, ListItem, ListState, Padding, Widget},
};

use crate::{
    AppCommand, AppResolver, AppTypes, Keys, ListRender, ListWidget, Popup, Widget as AppWidget,
};

pub struct KeysListPopup<A: AppTypes> {
    pub block: Block<'static>,
    pub state: KeysListRenderState<A>,
    pub list: ListWidget<KeysListRender<A>, A>,
}

impl<A: AppTypes> KeysListPopup<A> {
    pub fn new(title: String, keys: Keys<A>) -> Self {
        // State
        let mut list_state = ListState::default();
        list_state.select_first();

        let state = KeysListRenderState {
            state: list_state,
            keys,
        };

        // List
        let render = KeysListRender::default();
        let list = ListWidget::new(render);

        let block = Block::bordered()
            .title(format!(" {} ", title))
            .title_alignment(Alignment::Center)
            .padding(Padding::uniform(1))
            .border_type(BorderType::Rounded);

        Self { block, state, list }
    }
}

impl<A: AppTypes> AppWidget<A> for KeysListPopup<A> {
    type State = KeysListRenderState<A>;

    fn render(&mut self, root: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let area = self.render_area(root, state);
        let inner = self.block.inner(area);

        // Render
        self.block.clone().render(area, buf);
        self.list.render(inner, buf, state);
    }

    fn render_size(&self, area: Rect, state: &Self::State) -> Size {
        let mut size = self.list.render_size(area, state);

        size.width += 10;
        size.height += 5;

        size
    }

    fn on_command(
        &mut self,
        command: AppCommand<A>,
        resolver: AppResolver<A>,
        state: &mut Self::State,
    ) -> bool {
        self.list.on_command(command, resolver, state)
    }
}

impl<A: AppTypes> Popup<A> for KeysListPopup<A> {}

pub struct KeysListRender<A: AppTypes> {
    _marker: PhantomData<A>,
}

impl<A: AppTypes> Default for KeysListRender<A> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

pub struct KeysListRenderState<A: AppTypes> {
    state: ListState,
    keys: Keys<A>,
}

impl<A: AppTypes> ListRender<A> for KeysListRender<A> {
    type State = KeysListRenderState<A>;

    fn render_list(&self, _state: &Self::State) -> ratatui::widgets::List<'_> {
        List::default()
            .highlight_style(Style::default().fg(Color::Yellow))
            .highlight_symbol(">> ")
    }

    fn render_items(&self, state: &Self::State) -> impl Iterator<Item = ListItem<'_>> {
        state
            .keys
            .actions
            .iter()
            .map(|(name, _desc, _action)| ListItem::new(name.to_owned()))
    }

    fn render_state<'a>(&self, state: &'a mut Self::State) -> &'a mut ListState {
        &mut state.state
    }
}

impl<A: AppTypes> Clone for KeysListRenderState<A> {
    fn clone(&self) -> Self {
        Self {
            state: self.state,
            keys: self.keys.clone(),
        }
    }
}
