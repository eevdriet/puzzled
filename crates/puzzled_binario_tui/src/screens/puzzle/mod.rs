mod actions;
mod render;

use puzzled_binario::{Binario, BinarioState};
use puzzled_tui::{ActionHistory, GridRenderState};

pub struct PuzzleScreen {
    puzzle: Binario,
    solve_state: BinarioState,
    render_state: GridRenderState,

    commands: ActionHistory<BinarioState>,
}

impl PuzzleScreen {
    pub fn new(puzzle: Binario, solve_state: BinarioState, render_state: GridRenderState) -> Self {
        Self {
            puzzle,
            solve_state,
            render_state,

            commands: ActionHistory::default(),
        }
    }
}
