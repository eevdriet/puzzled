mod clues;
mod crossword;
mod state;

pub use clues::*;
pub use crossword::*;
pub use state::*;

use ratatui::{
    layout::{Constraint, Flex, Layout},
    prelude::{Buffer, Rect},
    widgets::{ListState, StatefulWidgetRef},
};

use puzzled_crossword::{ClueDirection, Crossword, CrosswordState};
use puzzled_tui::{
    Action, ActionBehavior, ActionHistory, ActionResolver, AppContext, Command, EventMode,
    FocusManager, GridRenderState, HandleCommand, RenderSize, StatefulScreen,
};

use crate::{AppState, CrosswordAction, CrosswordMotion};

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Focus {
    #[default]
    Crossword,

    Clues,

    Footer,
}

pub struct PuzzleScreen {
    state: PuzzleScreenState,

    // Widgets
    crossword: CrosswordWidget,
    clues: CluesWidget,
}

impl PuzzleScreen {
    pub fn new(
        puzzle: Crossword,
        solve_state: CrosswordState,
        render_state: GridRenderState,
    ) -> Self {
        let mut focus = FocusManager::from_mode_nodes([(Focus::Clues, EventMode::Normal)]);

        focus.link_right(Focus::Crossword, &[Focus::Clues]);
        focus.link_below(Focus::Footer, &[Focus::Clues]);

        let list = ListState::default().with_selected(Some(0));
        let state = PuzzleScreenState {
            puzzle,
            solve: solve_state,
            render: render_state,
            clue_dir: Some(ClueDirection::Across),
            across_down: list,
            across: list,
            down: list,
            history: ActionHistory::default(),
            focus,
        };

        Self {
            state,
            crossword: CrosswordWidget,
            clues: CluesWidget::default(),
        }
    }
}

impl StatefulScreen<CrosswordMotion, CrosswordAction, AppState> for PuzzleScreen {
    fn render(&mut self, root: Rect, buf: &mut Buffer, _state: &mut AppContext<AppState>) {
        // Compute sizes
        let gap = 2;

        let crossword_size = self.crossword.render_size(&self.state);
        let clues_size = self.clues.render_size(&self.state);

        let width = (crossword_size.width + gap + clues_size.width).min(root.width);

        let [area] = Layout::horizontal(vec![Constraint::Length(width)]).areas(root);

        // Have crossword be at most 50% and clues 25% of the area
        let crossword_width = crossword_size.width.min(5 * area.width / 10);

        // Clues on the right
        let [crossword, _, clues] = Layout::horizontal(vec![
            Constraint::Length(crossword_width),
            Constraint::Max(gap),
            Constraint::Fill(1),
        ])
        .areas(area);

        self.crossword.render_ref(crossword, buf, &mut self.state);
        self.clues.render_ref(clues, buf, &mut self.state);
    }

    fn on_command(
        &mut self,
        command: Command<CrosswordMotion, CrosswordAction>,
        resolver: ActionResolver<CrosswordMotion, CrosswordAction, AppState>,
        ctx: &mut AppContext<AppState>,
    ) -> bool {
        if let Some(action) = command.action() {
            match action {
                // Lifetime actions
                Action::Cancel => resolver.prev_screen(),
                Action::Quit => resolver.quit(),
                Action::Undo => self.state.history.undo(&mut self.state.solve),
                Action::Redo => self.state.history.redo(&mut self.state.solve),

                // Focus change actions
                action if action.is_focus() => {
                    return self
                        .state
                        .focus
                        .on_command(command, resolver, &mut ctx.mode);
                }
                _ => {}
            }
        }

        match self.state.focus.get() {
            Focus::Crossword => self
                .crossword
                .on_command(command, resolver, &mut self.state),
            Focus::Clues => self.clues.on_command(command, resolver, &mut self.state),
            Focus::Footer => self
                .crossword
                .on_command(command, resolver, &mut self.state),
        }
    }

    fn on_pause(&mut self, _ctx: &mut AppContext<AppState>) {
        self.state.solve.timer.pause();
    }

    fn on_resume(&mut self, _ctx: &mut AppContext<AppState>) {
        self.state.solve.timer.start();
    }
}
