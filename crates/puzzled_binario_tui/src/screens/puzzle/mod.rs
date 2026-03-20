mod binario;
mod state;

pub use binario::*;
pub use state::*;

use puzzled_binario::{Binario, BinarioState};
use puzzled_tui::{ActionHistory, AppContext, GridRenderState, Screen, Widget};
use ratatui::prelude::{Buffer, Rect};

use crate::BinarioApp;

pub struct PuzzleScreen {
    state: PuzzleScreenState,

    // Widgets
    binario: BinarioWidget,
}

impl PuzzleScreen {
    pub fn new(puzzle: Binario, solve_state: BinarioState, render_state: GridRenderState) -> Self {
        let state = PuzzleScreenState {
            puzzle,
            solve: solve_state,
            render: render_state,
            history: ActionHistory::default(),
        };

        Self {
            state,
            binario: BinarioWidget,
        }
    }
}

impl Screen<BinarioApp> for PuzzleScreen {
    fn render(&mut self, area: Rect, buf: &mut Buffer, _state: &mut AppContext<BinarioApp>) {
        self.binario.render(area, buf, &mut self.state);
    }

    fn on_pause(&mut self, _ctx: &mut AppContext<BinarioApp>) {
        self.state.solve.timer.pause();
    }

    fn on_resume(&mut self, _ctx: &mut AppContext<BinarioApp>) {
        self.state.solve.timer.start();
    }
}
