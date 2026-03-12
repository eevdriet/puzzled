mod focus;
// mod menu;

pub use focus::*;
// pub use menu::*;

use ratatui::{buffer::Buffer, layout::Rect};

use crate::{ActionResolver, AppContext, Command};

pub trait StatefulScreen<A, T, M, S> {
    // Rendering
    fn render(&mut self, area: Rect, buf: &mut Buffer, ctx: &mut AppContext<S>);

    // Actions
    fn on_command(
        &mut self,
        _command: Command<A, T, M>,
        _resolver: ActionResolver<A, T, M, S>,
        _ctx: &mut AppContext<S>,
    ) -> bool {
        false
    }

    // Lifetime events
    fn on_enter(&mut self, _ctx: &mut AppContext<S>) {}
    fn on_exit(&mut self, _ctx: &mut AppContext<S>) {}
    fn on_pause(&mut self, _ctx: &mut AppContext<S>) {}
    fn on_resume(&mut self, _ctx: &mut AppContext<S>) {}
}
