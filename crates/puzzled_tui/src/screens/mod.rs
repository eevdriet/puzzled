mod focus;
// mod menu;

pub use focus::*;
// pub use menu::*;

use ratatui::{buffer::Buffer, layout::Rect};

use crate::{AppCommand, AppContext, AppResolver, AppTypes, EventMode};

pub trait Screen<A: AppTypes> {
    // Rendering
    fn render(&mut self, area: Rect, buf: &mut Buffer, ctx: &mut AppContext<A>);

    fn on_tick(&self, _ctx: &AppContext<A>) -> bool {
        false
    }

    // Actions
    fn on_command(
        &mut self,
        _command: AppCommand<A>,
        _resolver: AppResolver<A>,
        _ctx: &mut AppContext<A>,
    ) -> bool {
        false
    }

    // Mode
    fn on_mode(
        &mut self,
        _mode: EventMode,
        _resolver: AppResolver<A>,
        _ctx: &mut AppContext<A>,
    ) -> bool {
        false
    }

    fn override_mode(&self) -> Option<EventMode> {
        None
    }

    // Lifetime events
    fn on_popup_open(&mut self, _ctx: &mut AppContext<A>) {}
    fn on_popup_close(&mut self, _ctx: &mut AppContext<A>) {}

    fn on_enter(&mut self, _ctx: &mut AppContext<A>) {}
    fn on_exit(&mut self, _ctx: &mut AppContext<A>) {}
    fn on_pause(&mut self, _ctx: &mut AppContext<A>) {}
    fn on_resume(&mut self, _ctx: &mut AppContext<A>) {}
}
