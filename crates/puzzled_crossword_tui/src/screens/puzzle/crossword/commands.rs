use puzzled_core::{Direction, Solve, SquareGridRef};
use puzzled_crossword::Solution;
use puzzled_tui::{
    Action, AppContext, Command, HandleBaseAction, HandleBaseMotion, HandleCommand, HandleOperator,
};

use crate::{
    AppState, CrosswordAction, CrosswordCommand, CrosswordMotion, CrosswordResolver,
    CrosswordTextObject, CrosswordWidget, PuzzleScreenState,
};

impl HandleCommand<CrosswordAction, CrosswordTextObject, CrosswordMotion, AppState>
    for CrosswordWidget
{
    type State = PuzzleScreenState;

    fn handle_command(
        &mut self,
        command: CrosswordCommand,
        _resolver: CrosswordResolver,
        _ctx: &mut AppContext<AppState>,
        state: &mut Self::State,
    ) -> bool {
        match command {
            Command::Motion { count, motion, op } => {
                let squares = SquareGridRef(state.puzzle.squares());
                let positions = squares.handle_base_motion(count, motion, &mut state.render);

                if let Some(op) = op {
                    state
                        .solve
                        .handle_operator(op, positions, &mut state.history);
                }

                true
            }
            Command::Action { action, .. } => self.handle_base_action(action, state),
            _ => false,
        }
    }
}

impl HandleBaseAction<CrosswordAction, PuzzleScreenState> for CrosswordWidget {
    fn handle_base_action(
        &mut self,
        action: Action<CrosswordAction>,
        state: &mut PuzzleScreenState,
    ) -> bool {
        let pos = state.render.cursor;
        let dir = match state.render.direction {
            Direction::Left | Direction::Right => Direction::Right,
            Direction::Up | Direction::Down => Direction::Down,
        };

        match action {
            Action::Insert(letter) => {
                let entry = Solution::Letter(letter.to_ascii_uppercase());
                state.solve.enter(&pos, entry);

                if let Some(next) = pos + dir
                    && state.puzzle.squares().get_fill(next).is_some()
                {
                    state.render.cursor = next;
                }
            }

            Action::DeleteLeft => {
                state.solve.clear(&pos);

                if let Some(next) = pos - dir
                    && state.puzzle.squares().get_fill(next).is_some()
                {
                    state.render.cursor = next;
                }
            }

            Action::DeleteRight => {
                state.solve.clear(&pos);
            }

            _ => return false,
        }

        true
    }
}
