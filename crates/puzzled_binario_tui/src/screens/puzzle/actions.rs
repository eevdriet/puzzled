use puzzled_tui::{ActionResolver, Command, HandleCommand};

use crate::{AppState, BinarioAction, PuzzleScreen};

impl HandleCommand<BinarioAction, AppState> for PuzzleScreen {
    type State = AppState;

    fn on_command(
        &mut self,
        _command: Command<BinarioAction>,
        _resolver: ActionResolver<BinarioAction, AppState>,
        _state: &mut Self::State,
    ) -> bool {
        false
    }
}
