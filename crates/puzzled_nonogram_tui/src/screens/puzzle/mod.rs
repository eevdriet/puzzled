mod state;

pub use state::*;

use puzzled_nonogram::{Nonogram, NonogramState};
use puzzled_tui::{Action, ActionHistory, AppContext, Command, GridRenderState, Screen};
use ratatui::prelude::{Buffer, Rect};

use crate::NonogramApp;

pub struct PuzzleScreen {
    pub state: PuzzleScreenState,
    // Widgets
}

impl PuzzleScreen {
    pub fn new(puzzle: Nonogram, solve: NonogramState, render: GridRenderState) -> Self {
        let state = PuzzleScreenState {
            puzzle,
            solve,
            render,
            history: ActionHistory::default(),
        };

        Self { state }
    }
}

impl Screen<NonogramApp> for PuzzleScreen {
    fn render(&mut self, _area: Rect, _buf: &mut Buffer, _ctx: &mut AppContext<NonogramApp>) {}

    fn on_command(
        &mut self,
        command: puzzled_tui::AppCommand<NonogramApp>,
        resolver: puzzled_tui::AppResolver<NonogramApp>,
        _ctx: &mut AppContext<NonogramApp>,
    ) -> bool {
        let mut handled_action = false;

        if let Command::Action { count, action } = &command {
            handled_action = true;

            match action {
                // Lifetime actions
                Action::Cancel => resolver.prev_screen(),
                Action::ShowHelp => resolver.open_popup(),
                Action::Quit => resolver.quit(),
                Action::Undo => self.state.history.undo(*count, &mut self.state.solve),
                Action::Redo => self.state.history.redo(*count, &mut self.state.solve),
                _ => {
                    handled_action = false;
                }
            }
        }

        handled_action
    }
}
