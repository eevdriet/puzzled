use puzzled_binario::{Binario, BinarioState};
use puzzled_core::GridState;
use puzzled_tui::{ActionHistory, SidedGridRenderState};

pub struct PuzzleScreenState {
    // Binario state
    pub puzzle: Binario,
    pub solve: BinarioState,
    pub render: SidedGridRenderState,

    // Commands
    pub history: ActionHistory<GridState<Binario>>,
}
