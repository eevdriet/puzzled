use puzzled_core::{Direction, Solve, SquareGridRef};
use puzzled_crossword::Solution;
use puzzled_tui::{Action, ActionResolver, Command, HandleCommand};

use crate::{AppState, CrosswordAction, CrosswordMotion, CrosswordWidget, PuzzleScreenState};

impl HandleCommand<CrosswordMotion, CrosswordAction, AppState> for CrosswordWidget {
    type State = PuzzleScreenState;

    fn on_command(
        &mut self,
        command: Command<CrosswordMotion, CrosswordAction>,
        resolver: ActionResolver<CrosswordMotion, CrosswordAction, AppState>,
        state: &mut Self::State,
    ) -> bool {
        // Handle general grid commands first
        let mut grid = SquareGridRef(&state.solve.entries);
        if grid.on_command(command.clone(), resolver, &mut state.render) {
            state.update_clues_from_cursor();
            return true;
        }

        // Then handle crossword specific input
        let Some(action) = command.action() else {
            return false;
        };

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

            _ => return false,
        }

        true
    }
}
