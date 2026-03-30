mod nonogram;
mod state;

pub use nonogram::*;
pub use state::*;

use puzzled_nonogram::{Fill, Nonogram, NonogramState};
use puzzled_tui::{
    Action, ActionHistory, AppCommand, AppContext, AppResolver, Command, EventMode, HandleMode,
    Screen, SidedGridRenderState, Widget,
};
use ratatui::prelude::{Buffer, Rect};

use crate::NonogramApp;

pub struct PuzzleScreen {
    pub state: PuzzleScreenState,

    // Widgets
    nonogram: NonogramWidget,
}

impl PuzzleScreen {
    pub fn new(puzzle: Nonogram, solve: NonogramState, render: SidedGridRenderState) -> Self {
        let state = PuzzleScreenState {
            puzzle,
            solve,
            render,
            fill: Fill::Color(b'a' as u32),
            history: ActionHistory::default(),
        };

        Self {
            state,
            nonogram: NonogramWidget,
        }
    }
}

impl Screen<NonogramApp> for PuzzleScreen {
    fn render(&mut self, area: Rect, buf: &mut Buffer, ctx: &mut AppContext<NonogramApp>) {
        self.nonogram.render(area, buf, ctx, &mut self.state);
    }

    fn on_command(
        &mut self,
        command: AppCommand<NonogramApp>,
        resolver: AppResolver<NonogramApp>,
        ctx: &mut AppContext<NonogramApp>,
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
            || self
                .nonogram
                .on_command(command, resolver, ctx, &mut self.state)
    }

    fn on_mode(
        &mut self,
        mode: EventMode,
        resolver: AppResolver<NonogramApp>,
        ctx: &mut AppContext<NonogramApp>,
    ) -> bool {
        let solutions = &mut self.state.solve.state.solutions;
        solutions.handle_mode(mode, resolver, ctx, &mut self.state.render.grid)
    }

    fn override_mode(&self) -> Option<EventMode> {
        self.nonogram.override_mode()
    }

    fn on_enter(&mut self, _ctx: &mut AppContext<NonogramApp>) {
        self.state.solve.timer.start();
    }

    fn on_pause(&mut self, _ctx: &mut AppContext<NonogramApp>) {
        self.state.solve.timer.pause();
    }

    fn on_resume(&mut self, _ctx: &mut AppContext<NonogramApp>) {
        self.state.solve.timer.start();
    }
}
