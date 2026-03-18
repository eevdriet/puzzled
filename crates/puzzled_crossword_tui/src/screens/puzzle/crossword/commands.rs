use puzzled_core::{Direction, Solve, SquareGridRef};
use puzzled_crossword::Solution;
use puzzled_tui::{
    Action, AppContext, AsCore, Command, EventMode, HandleBaseAction, HandleBaseMotion,
    HandleCommand, HandleOperator, Operator,
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
        resolver: CrosswordResolver,
        _ctx: &mut AppContext<AppState>,
        state: &mut Self::State,
    ) -> bool {
        match command {
            Command::Operator(op) => {
                if state.render.mode.is_visual() {
                    let size = state.puzzle.squares().size();
                    let positions = state
                        .render
                        .selection
                        .range(size)
                        .positions()
                        .map(|pos| pos.as_core());

                    state
                        .solve
                        .handle_operator(op, positions, &mut state.history);

                    let mode = match op {
                        Operator::Change => EventMode::Insert,
                        _ => EventMode::Normal,
                    };
                    resolver.set_mode(mode);

                    true
                } else if !op.requires_motion() {
                    let positions = vec![state.render.cursor];

                    state
                        .solve
                        .handle_operator(op, positions, &mut state.history);
                    true
                } else {
                    false
                }
            }
            Command::Motion { count, motion, op } if state.render.mode.is_visual() => {
                tracing::info!("Visual motion: {motion:?}");
                assert!(op.is_none());

                let squares = SquareGridRef(state.puzzle.squares());
                let positions = squares.handle_base_motion(count, motion, &mut state.render);

                if let Some(end) = positions.into_iter().last() {
                    state.render.selection.update(end);
                }

                true
            }
            Command::Motion { count, motion, op } => {
                tracing::info!("Other motion: {motion:?}");

                {
                    let squares = SquareGridRef(state.puzzle.squares());
                    let positions = squares.handle_base_motion(count, motion, &mut state.render);

                    if let Some(op) = op {
                        state
                            .solve
                            .handle_operator(op, positions, &mut state.history);
                    }
                }

                state.update_clues_from_cursor();
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
