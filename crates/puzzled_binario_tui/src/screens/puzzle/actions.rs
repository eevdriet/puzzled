use puzzled_tui::HandleCommand;

use crate::{
    AppState, BinarioAction, BinarioCommand, BinarioContext, BinarioResolver, PuzzleScreen,
};

impl HandleCommand<BinarioAction, (), (), AppState> for PuzzleScreen {
    type State = AppState;

    fn handle_command(
        &mut self,
        _command: BinarioCommand,
        _resolver: BinarioResolver,
        _ctx: &mut BinarioContext,
        _state: &mut Self::State,
    ) -> bool {
        false
    }
}
