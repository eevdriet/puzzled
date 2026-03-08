use puzzled_tui::{Action, ActionResolver, HandleAction};

use crate::{AppState, BinarioAction, PuzzleScreen};

impl HandleAction<BinarioAction, AppState> for PuzzleScreen {
    type State = AppState;

    fn on_action(
        &mut self,
        action: Action<BinarioAction>,
        resolver: ActionResolver<BinarioAction, AppState>,
        _state: &mut Self::State,
    ) {
        match action {
            Action::Cancel => resolver.prev_screen(),
            Action::Quit => resolver.quit(),

            action => {
                let mut grid = self.solve_state.to_merged();
                grid.on_action(action, resolver, &mut self.render_state);
            }
        }
    }
}
