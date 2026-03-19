mod actions;
mod render;

use puzzled_binario::{Binario, BinarioState};
use puzzled_tui::{ActionHistory, GridRenderState, StatefulScreen};
use ratatui::prelude::{Buffer, Rect};

use crate::{AppState, BinarioAction, BinarioContext};

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

impl StatefulScreen<BinarioAction, (), (), AppState> for PuzzleScreen {
    fn render(&mut self, _area: Rect, _buf: &mut Buffer, _state: &mut BinarioContext) {}

    fn on_pause(&mut self, _ctx: &mut BinarioContext) {
        self.solve_state.timer.pause();
    }

    fn on_resume(&mut self, _ctx: &mut BinarioContext) {
        self.solve_state.timer.start();
    }
}
