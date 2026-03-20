mod focus;
// mod menu;

pub use focus::*;
// pub use menu::*;

use ratatui::{buffer::Buffer, layout::Rect};

use crate::{AppContext, AppResolver, Command, EventMode};

pub trait Screen<A, T, M, S> {
    // Rendering
    fn render(&mut self, area: Rect, buf: &mut Buffer, ctx: &mut AppContext<A, T, M, S>);

    fn on_tick(&self, _ctx: &AppContext<A, T, M, S>) -> bool {
        false
    }

    // Actions
    fn on_command(
        &mut self,
        _command: Command<A, T, M>,
        _resolver: AppResolver<A, T, M, S>,
        _ctx: &mut AppContext<A, T, M, S>,
    ) -> bool {
        false
    }

    // Mode
    fn on_mode(
        &mut self,
        _mode: EventMode,
        _resolver: AppResolver<A, T, M, S>,
        _ctx: &mut AppContext<A, T, M, S>,
    ) -> bool {
        false
    }

    fn override_mode(&self) -> Option<EventMode> {
        None
    }

    // Lifetime events
    fn on_popup_open(&mut self, _ctx: &mut AppContext<A, T, M, S>) {}
    fn on_popup_close(&mut self, _ctx: &mut AppContext<A, T, M, S>) {}

    fn on_enter(&mut self, _ctx: &mut AppContext<A, T, M, S>) {}
    fn on_exit(&mut self, _ctx: &mut AppContext<A, T, M, S>) {}
    fn on_pause(&mut self, _ctx: &mut AppContext<A, T, M, S>) {}
    fn on_resume(&mut self, _ctx: &mut AppContext<A, T, M, S>) {}
}
