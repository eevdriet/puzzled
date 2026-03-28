use puzzled_nonogram::{Nonogram, NonogramState};
use puzzled_tui::{ActionHistory, GridRenderState};

pub struct PuzzleScreenState {
    // Binario state
    pub puzzle: Nonogram,
    pub solve: NonogramState,
    pub render: GridRenderState,

    // Commands
    pub history: ActionHistory<NonogramState>,
}
