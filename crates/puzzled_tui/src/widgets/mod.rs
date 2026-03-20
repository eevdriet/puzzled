mod board;
mod keys;
mod list;
mod timer;

// mod settings;

pub use board::*;
pub use keys::*;
pub use list::*;
pub use timer::*;
// pub use settings::*;

use ratatui::{
    buffer::Buffer,
    layout::{Rect, Size},
};

use crate::{AppContext, AppResolver, Command, EventMode};

pub trait RenderSize<S> {
    fn render_size(&self, state: &S) -> Size;
}

pub trait Widget<A, T, M, S> {
    type State;

    // Rendering
    fn render(
        &self,
        area: Rect,
        buf: &mut Buffer,
        ctx: &mut AppContext<A, T, M, S>,
        state: &mut Self::State,
    );

    fn render_size(&self, state: &Self::State) -> Size;

    // Commands
    fn on_command(
        &mut self,
        _command: Command<A, T, M>,
        _resolver: AppResolver<A, T, M, S>,
        _ctx: &mut AppContext<A, T, M, S>,
        _state: &mut Self::State,
    ) -> bool {
        false
    }

    // Mode
    fn override_mode(&self) -> Option<EventMode> {
        None
    }

    fn on_mode(
        &mut self,
        _mode: EventMode,
        _resolver: AppResolver<A, T, M, S>,
        _ctx: &mut AppContext<A, T, M, S>,
        _state: &mut Self::State,
    ) -> bool {
        false
    }

    // Lifetime events
    fn on_enter(&mut self, _ctx: &mut AppContext<A, T, M, S>, _state: &mut Self::State) {}
    fn on_exit(&mut self, _ctx: &mut AppContext<A, T, M, S>, _state: &mut Self::State) {}
    fn on_pause(&mut self, _ctx: &mut AppContext<A, T, M, S>, _state: &mut Self::State) {}
    fn on_resume(&mut self, _ctx: &mut AppContext<A, T, M, S>, _state: &mut Self::State) {}
}
