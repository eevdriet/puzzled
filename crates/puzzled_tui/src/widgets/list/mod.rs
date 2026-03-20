use crossterm::event::MouseEventKind;
use ratatui::buffer::Buffer;
use ratatui::layout::{Rect, Size};
use ratatui::widgets::{List, ListItem, ListState, Widget};

use crate::{AppResolver, Command, Motion, Widget as AppWidget};

pub trait ListRender {
    type State;

    fn render_items(&self, state: &Self::State) -> impl Iterator<Item = ListItem<'_>>;
}

pub struct ListWidget<R> {
    render: R,
}

impl<R> ListWidget<R> {
    pub fn new(render: R) -> Self {
        Self { render }
    }
}

pub struct ListContext<S> {
    pub state: S,
    pub list: ListState,
}

impl<R, A, T, M, S> AppWidget<A, T, M, S> for ListWidget<R>
where
    R: ListRender,
{
    type State = ListContext<R::State>;

    fn render(&mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let items = self.render.render_items(&state.state);
        List::new(items).highlight_symbol(">> ").render(area, buf);
    }

    fn render_size(&self, state: &Self::State) -> Size {
        self.render
            .render_items(&state.state)
            .fold(Size::ZERO, |mut size, item| {
                size.width = size.width.max(item.width() as u16);
                size.height += item.height() as u16;

                size
            })
    }

    fn on_command(
        &mut self,
        command: Command<A, T, M>,
        _resolver: AppResolver<A, T, M, S>,
        state: &mut Self::State,
    ) -> bool {
        match command {
            Command::Motion { count, motion, .. } => {
                let list = &mut state.list;
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
