use puzzled_nonogram::{Fill, Nonogram, NonogramState};
use puzzled_tui::{ActionHistory, SidedGridRenderState};

pub struct PuzzleScreenState {
    // Nonogram state
    pub puzzle: Nonogram,
    pub solve: NonogramState,
    pub render: SidedGridRenderState,

    pub fill: Fill,

    // Commands
    pub history: ActionHistory<NonogramState>,
}
