mod focus;
// mod menu;

pub use focus::*;
// pub use menu::*;

use ratatui::{buffer::Buffer, layout::Rect};

use crate::{ActionResolver, Command};

pub trait StatefulScreen<A, S> {
    // Rendering
    fn render(&mut self, area: Rect, buf: &mut Buffer, state: &mut S);

    // Actions
    fn on_command(
        &mut self,
        _command: Command<A>,
        _resolver: ActionResolver<A, S>,
        _state: &mut S,
    ) -> bool {
        false
    }

    // Lifetime events
    fn on_enter(&mut self, _state: &mut S) {}
    fn on_exit(&mut self, _state: &mut S) {}
    fn on_pause(&mut self, _state: &mut S) {}
    fn on_resume(&mut self, _state: &mut S) {}
}
