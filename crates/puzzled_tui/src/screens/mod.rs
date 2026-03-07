mod menu;

pub use menu::*;

use ratatui::Frame;

use crate::{Action, ActionResolver, AppEvent};

pub trait StatefulScreen<A, S> {
    fn render(&mut self, frame: &mut Frame, state: &S);

    // Actions
    fn on_action(&mut self, _action: Action<A>, _resolver: ActionResolver<A, S>, _state: &mut S) {}
    fn on_event(&mut self, _event: AppEvent, _resolver: ActionResolver<A, S>, _state: &mut S) {}

    // Lifetime events
    fn on_enter(&mut self, _state: &mut S) {}
    fn on_exit(&mut self, _state: &mut S) {}
    fn on_pause(&mut self, _state: &mut S) {}
    fn on_resume(&mut self, _state: &mut S) {}
}
