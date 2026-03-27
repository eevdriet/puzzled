mod board;
mod keys;
mod timer;
mod ui;

// mod settings;

pub use board::*;
pub use keys::*;
pub use timer::*;
pub use ui::*;
// pub use settings::*;

use ratatui::{
    buffer::Buffer,
    layout::{Rect, Size},
};

use crate::{AppCommand, AppContext, AppResolver, AppTypes, EventMode, center_area};

pub trait Widget<A: AppTypes> {
    type State;

    // Rendering
    fn render(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        ctx: &mut AppContext<A>,
        state: &mut Self::State,
    );

    fn render_size(&self, area: Rect, _ctx: &AppContext<A>, _state: &Self::State) -> Size {
        area.as_size()
    }

    fn render_area(&self, area: Rect, ctx: &AppContext<A>, state: &Self::State) -> Rect {
        center_area(area, self.render_size(area, ctx, state))
    }

    fn on_tick(&self, _ctx: AppContext<A>, _state: &Self::State) -> bool {
        false
    }

    // Commands
    fn on_command(
        &mut self,
        _command: AppCommand<A>,
        _resolver: AppResolver<A>,
        _ctx: &mut AppContext<A>,
        _state: &mut Self::State,
    ) -> bool {
        false
    }

    // Mode
    fn on_mode(
        &mut self,
        _mode: EventMode,
        _resolver: AppResolver<A>,
        _ctx: &mut AppContext<A>,
        _state: &mut Self::State,
    ) -> bool {
        false
    }

    fn override_mode(&self) -> Option<EventMode> {
        None
    }

    // Lifetime events
    fn on_enter(&mut self, _state: &mut Self::State) {}
    fn on_exit(&mut self, _state: &mut Self::State) {}
    fn on_pause(&mut self, _state: &mut Self::State) {}
    fn on_resume(&mut self, _state: &mut Self::State) {}
}
