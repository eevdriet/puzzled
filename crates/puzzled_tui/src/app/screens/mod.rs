use ratatui::Frame;

use crate::{Action, ActionResolver};

pub trait StatefulScreen<A, S> {
    fn render(&mut self, frame: &mut Frame, state: &S);

    // Actions
    fn on_action(&mut self, action: Action<A>, resolver: ActionResolver<A, S>, state: &mut S);

    // Lifetime events
    fn on_enter(&mut self, _state: &mut S) {}
    fn on_exit(&mut self, _state: &mut S) {}
    fn on_pause(&mut self, _state: &mut S) {}
    fn on_resume(&mut self, _state: &mut S) {}
}
