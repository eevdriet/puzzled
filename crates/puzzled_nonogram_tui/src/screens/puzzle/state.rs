use puzzled_nonogram::{Fill, Nonogram, NonogramState};
use puzzled_tui::{ActionHistory, GridRenderState};

pub struct PuzzleScreenState {
    // Nonogram state
    pub puzzle: Nonogram,
    pub solve: NonogramState,
    pub render: GridRenderState,

    pub fill: Fill,

    // Commands
    pub history: ActionHistory<NonogramState>,
}
