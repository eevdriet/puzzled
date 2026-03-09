mod actions;
mod render;

use puzzled_binario::{Binario, BinarioState};
use puzzled_tui::{CommandHistory, GridRenderState, StatefulScreen};
use ratatui::prelude::{Buffer, Rect};

use crate::{AppState, BinarioAction};

pub struct PuzzleScreen {
    puzzle: Binario,
    solve_state: BinarioState,
    render_state: GridRenderState,

    commands: CommandHistory<BinarioState>,
}

impl PuzzleScreen {
    pub fn new(puzzle: Binario, solve_state: BinarioState, render_state: GridRenderState) -> Self {
        Self {
            puzzle,
            solve_state,
            render_state,

            commands: CommandHistory::default(),
        }
    }
}

impl StatefulScreen<(), BinarioAction, AppState> for PuzzleScreen {
    fn render(&mut self, area: Rect, buf: &mut Buffer, state: &mut AppState) {}

    fn on_pause(&mut self, _state: &mut AppState) {
        self.solve_state.timer.pause();
    }

    fn on_resume(&mut self, _state: &mut AppState) {
        self.solve_state.timer.start();
    }
}
