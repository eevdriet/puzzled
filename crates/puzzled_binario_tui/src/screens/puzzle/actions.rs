use puzzled_tui::{ActionResolver, AppContext, Command, HandleCommand};

use crate::{AppState, BinarioAction, PuzzleScreen};

impl HandleCommand<(), BinarioAction, AppState> for PuzzleScreen {
    type State = AppState;

    fn handle_command(
        &mut self,
        _command: Command<(), BinarioAction>,
        _resolver: ActionResolver<(), BinarioAction, AppState>,
        _ctx: &mut AppContext<AppState>,
        _state: &mut Self::State,
    ) -> bool {
        false
    }
}
