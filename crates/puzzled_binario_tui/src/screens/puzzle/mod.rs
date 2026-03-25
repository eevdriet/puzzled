mod binario;
mod state;

pub use binario::*;
pub use state::*;

use puzzled_binario::{Binario, BinarioState};
use puzzled_tui::{
    Action, ActionHistory, AppCommand, AppContext, AppResolver, Command, HandleMode, Screen,
    SidedGridRenderState, Widget,
};
use ratatui::prelude::{Buffer, Rect};

use crate::BinarioApp;

pub struct PuzzleScreen {
    state: PuzzleScreenState,

    // Widgets
    binario: BinarioWidget,
}

impl PuzzleScreen {
    pub fn new(
        puzzle: Binario,
        solve_state: BinarioState,
        render_state: SidedGridRenderState,
    ) -> Self {
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

    fn on_tick(&self, _ctx: &AppContext<BinarioApp>) -> bool {
        true
    }

    fn on_command(
        &mut self,
        command: AppCommand<BinarioApp>,
        resolver: AppResolver<BinarioApp>,
        _ctx: &mut AppContext<BinarioApp>,
    ) -> bool {
        let mut handled_action = false;

        if let Command::Action { count, action } = &command {
            handled_action = true;

            match action {
                // Lifetime actions
                Action::Cancel => resolver.prev_screen(),
                Action::ShowHelp => resolver.open_popup(),
                Action::Quit => resolver.quit(),
                Action::Undo => self.state.history.undo(*count, &mut self.state.solve.state),
                Action::Redo => self.state.history.redo(*count, &mut self.state.solve.state),
                _ => {
                    handled_action = false;
                }
            }
        }

        handled_action || self.binario.on_command(command, resolver, &mut self.state)
    }

    fn on_mode(
        &mut self,
        mode: puzzled_tui::EventMode,
        resolver: AppResolver<BinarioApp>,
        ctx: &mut AppContext<BinarioApp>,
    ) -> bool {
        let solutions = &mut self.state.solve.state.solutions;
        solutions.handle_mode(mode, resolver, ctx, &mut self.state.render.grid)
    }

    fn override_mode(&self) -> Option<puzzled_tui::EventMode> {
        self.binario.override_mode()
    }

    fn on_enter(&mut self, _ctx: &mut AppContext<BinarioApp>) {
        self.state.solve.state.timer.start();
    }

    fn on_pause(&mut self, _ctx: &mut AppContext<BinarioApp>) {
        self.state.solve.state.timer.pause();
    }

    fn on_resume(&mut self, _ctx: &mut AppContext<BinarioApp>) {
        self.state.solve.state.timer.start();
    }
}
