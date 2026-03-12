use puzzled_tui::{AppContext, HandleCommand};

use crate::{AppState, BinarioAction, BinarioCommand, BinarioResolver, PuzzleScreen};

impl HandleCommand<BinarioAction, (), (), AppState> for PuzzleScreen {
    type State = AppState;

    fn handle_command(
        &mut self,
        _command: BinarioCommand,
        _resolver: BinarioResolver,
        _ctx: &mut AppContext<AppState>,
        _state: &mut Self::State,
    ) -> bool {
        false
    }
}
