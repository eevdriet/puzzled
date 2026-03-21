use std::marker::PhantomData;

use crossterm::event::MouseEventKind;
use ratatui::buffer::Buffer;
use ratatui::layout::{Rect, Size};
use ratatui::widgets::{List, ListItem, ListState, StatefulWidget};

use crate::{AppCommand, AppResolver, AppTypes, Command, Motion, Widget as AppWidget};

pub trait ListRender {
    type State;

    fn render_list(&self, state: &Self::State) -> List<'_>;
    fn render_items(&self, state: &Self::State) -> impl Iterator<Item = ListItem<'_>>;
    fn render_state<'a>(&self, state: &'a mut Self::State) -> &'a mut ListState;
}

pub struct ListWidget<R, A> {
    pub render: R,
    pub _marker: PhantomData<A>,
}

impl<R, A> ListWidget<R, A> {
    pub fn new(render: R) -> Self {
        Self {
            render,
            _marker: PhantomData,
        }
    }
}

// pub struct ListContext<S> {
//     pub state: S,
//     pub list: ListState,
// }

impl<R, A> AppWidget<A> for ListWidget<R, A>
where
    R: ListRender,
    A: AppTypes,
{
    type State = R::State;

    fn render(&mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let items = self.render.render_items(state);
        let list = self.render.render_list(state).items(items);
        let state = self.render.render_state(state);

        list.render(area, buf, state);
    }

    fn render_size(&self, state: &Self::State) -> Size {
        let items = self.render.render_items(state);

        items.fold(Size::ZERO, |mut size, item| {
            size.width = size.width.max(item.width() as u16);
            size.height += item.height() as u16;

            size
        })
    }

    fn on_command(
        &mut self,
        command: AppCommand<A>,
        _resolver: AppResolver<A>,
        state: &mut Self::State,
    ) -> bool {
        match command {
            Command::Motion { count, motion, .. } => {
                let list = self.render.render_state(state);
                let count = count as u16;

                match motion {
                    Motion::Mouse(mouse) => match mouse.kind {
                        MouseEventKind::ScrollDown => list.scroll_down_by(count),
                        MouseEventKind::ScrollUp => list.scroll_up_by(count),
                        _ => return false,
                    },
                    Motion::ColStart => list.select_first(),
                    Motion::ColEnd => list.select_last(),
                    Motion::Down => list.scroll_down_by(count),
                    Motion::Up => list.scroll_up_by(count),
                    _ => return false,
                }
            }
            _ => return false,
        }

        true
    }
}
