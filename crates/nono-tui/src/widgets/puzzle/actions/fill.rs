use nono::{Fill, Position};

use crate::{ActionOutcome, ActionResult, AppState, UndoAction};

#[derive(Debug, Clone)]
pub struct FillAction {
    changes: Vec<CellChange>,
}

impl FillAction {
    pub fn new(changes: Vec<CellChange>) -> Self {
        Self { changes }
    }
}

#[derive(Debug, Clone)]
pub struct CellChange {
    pos: Position,
    before: Fill,
    after: Fill,
}

impl CellChange {
    pub fn new(pos: Position, before: Fill, after: Fill) -> Self {
        Self { pos, before, after }
    }
}

impl UndoAction for FillAction {
    fn execute(&mut self, state: &mut AppState) -> ActionResult {
        for change in &self.changes {
            let puzzle = &mut state.puzzle.puzzle;

            // Give the cell the correct fill
            puzzle.fill_cell(change.pos, change.after);

            // Then update the cell state in the solver
            state.solver.update_cell(puzzle, change.pos, change.after);
        }

        Ok(ActionOutcome::Consumed)
    }

    fn undo(&mut self, state: &mut AppState) -> ActionResult {
        for change in &self.changes {
            let puzzle = &mut state.puzzle.puzzle;

            // Give the cell the correct fill
            puzzle.fill_cell(change.pos, change.before);

            // Then update the cell state in the solver
            state.solver.update_cell(puzzle, change.pos, change.before);
        }

        Ok(ActionOutcome::Consumed)
    }
}
