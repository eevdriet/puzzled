use puzzled_binario::{Binario, BinarioState, Bit};
use puzzled_tui::{ActionHistory, SidedGridRenderState};

pub struct PuzzleScreenState {
    // Binario state
    pub puzzle: Binario,
    pub solve: BinarioState,
    pub render: SidedGridRenderState,

    pub bit: Bit,

    // Commands
    pub history: ActionHistory<BinarioState>,
}
