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

use crate::{AppResolver, Command, EventMode};

pub trait RenderSize<S> {
    fn render_size(&self, state: &S) -> Size;
}

pub trait Widget<A, T, M, S> {
    type State;

    // Rendering
    fn render(&mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State);

    fn render_size(&self, state: &Self::State) -> Size;

    // Commands
    fn on_command(
        &mut self,
        _command: Command<A, T, M>,
        _resolver: AppResolver<A, T, M, S>,
        _state: &mut Self::State,
    ) -> bool {
        false
    }

    // Mode
    fn on_mode(
        &mut self,
        _mode: EventMode,
        _resolver: AppResolver<A, T, M, S>,
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
