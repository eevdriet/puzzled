use puzzled_crossword::ClueDirection;
use puzzled_tui::{Action, ActionResolver, HandleAction};
use ratatui::{
    prelude::{Buffer, Rect},
    widgets::StatefulWidgetRef,
};

use crate::{AppState, CrosswordAction, PuzzleScreenState};

pub struct CluesWidget {
    direction: ClueDirection,
}

impl CluesWidget {
    pub fn new(direction: ClueDirection) -> Self {
        Self { direction }
    }
}

impl StatefulWidgetRef for CluesWidget {
    type State = PuzzleScreenState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {}
}

impl HandleAction<CrosswordAction, AppState> for CluesWidget {
    type State = PuzzleScreenState;

    fn on_action(
        &mut self,
        _action: Action<CrosswordAction>,
        _resolver: ActionResolver<CrosswordAction, AppState>,
        _state: &mut Self::State,
    ) {
    }
}
