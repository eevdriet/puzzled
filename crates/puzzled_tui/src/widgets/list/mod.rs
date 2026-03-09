use ratatui::buffer::Buffer;
use ratatui::layout::{Rect, Size};
use ratatui::widgets::{List, ListItem, ListState, StatefulWidgetRef, Widget};

use crate::RenderSize;

pub trait ListRender {
    type State;

    fn render_items(&self, state: &Self::State) -> impl Iterator<Item = ListItem<'_>>;
}

pub struct ListWidget<R>(R);

pub struct ListContext<S> {
    pub state: S,
    pub list: ListState,
}

impl<R> StatefulWidgetRef for ListWidget<R>
where
    R: ListRender,
{
    type State = ListContext<R::State>;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, ctx: &mut Self::State) {
        let items = self.0.render_items(&ctx.state);
        List::new(items).highlight_symbol(">> ").render(area, buf);
    }
}

impl<R> RenderSize<R::State> for ListWidget<R>
where
    R: ListRender,
{
    fn render_size(&self, state: &R::State) -> ratatui::prelude::Size {
        self.0
            .render_items(state)
            .fold(Size::ZERO, |mut size, item| {
                size.width = size.width.max(item.width() as u16);
                size.height += item.height() as u16;

                size
            })
    }
}
