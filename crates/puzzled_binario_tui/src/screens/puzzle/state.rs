use puzzled_binario::{Binario, BinarioState, Bit};
use puzzled_core::GridState;
use puzzled_tui::{ActionHistory, GridRenderState};

pub struct PuzzleScreenState {
    // Binario state
    pub puzzle: Binario,
    pub solve: BinarioState,
    pub render: GridRenderState,

    // Commands
    pub history: ActionHistory<GridState<Bit>>,
}
