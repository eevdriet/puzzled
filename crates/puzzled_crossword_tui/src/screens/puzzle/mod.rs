mod clues;
mod crossword;
mod footer;
mod state;

pub use clues::*;
pub use crossword::*;
pub use footer::*;
pub use state::*;

use ratatui::{
    layout::{Constraint, Layout},
    prelude::{Buffer, Rect},
    widgets::{ListState, StatefulWidgetRef},
};

use puzzled_crossword::{ClueDirection, Crossword, CrosswordState};
use puzzled_tui::{
    Action, ActionBehavior, ActionHistory, ActionResolver, AppContext, Command, EventMode,
    FocusManager, GridRenderState, HandleBaseAction, HandleCommand, RenderSize, StatefulScreen,
};

use crate::{
    AppState, CrosswordAction, CrosswordCommand, CrosswordMotion, CrosswordResolver,
    CrosswordTextObject,
};

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
    footer: FooterWidget,
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
            footer: FooterWidget,
        }
    }
}

impl StatefulScreen<CrosswordAction, CrosswordTextObject, CrosswordMotion, AppState>
    for PuzzleScreen
{
    fn render(&mut self, root: Rect, buf: &mut Buffer, state: &mut AppContext<AppState>) {
        // Compute sizes
        let gap = 2;

        let crossword_size = self.crossword.render_size(&self.state);
        let clues_size = self.clues.render_size(&self.state);

        let width = (crossword_size.width + gap + clues_size.width).min(root.width);

        let [area] = Layout::horizontal(vec![Constraint::Length(width)]).areas(root);

        // Have crossword be at most 50% and clues 25% of the area
        let crossword_width = crossword_size.width.min(5 * area.width / 10);

        // Clues and footer on the right
        let [crossword, _, right] = Layout::horizontal(vec![
            Constraint::Length(crossword_width),
            Constraint::Max(gap),
            Constraint::Fill(1),
        ])
        .areas(area);

        let [clues, _, footer] = Layout::vertical(vec![
            Constraint::Fill(1),
            Constraint::Length(gap),
            Constraint::Length(5),
        ])
        .areas(right);

        tracing::debug!("Root: {root:?}");
        tracing::debug!("Crossword: {crossword:?}");
        tracing::debug!("Right: {right:?}");
        tracing::debug!("Clues: {clues:?}");
        tracing::debug!("Footer: {footer:?}");

        // Render
        self.crossword.render_ref(crossword, buf, &mut self.state);
        self.clues.render_ref(clues, buf, &mut self.state);

        let mut footer_state = FooterState { mode: state.mode };
        self.footer.render_ref(footer, buf, &mut footer_state);
    }

    fn on_command(
        &mut self,
        command: CrosswordCommand,
        resolver: CrosswordResolver,
        ctx: &mut AppContext<AppState>,
    ) -> bool {
        if let Command::Action { action, count } = &command {
            match action {
                // Lifetime actions
                Action::Cancel => resolver.prev_screen(),
                Action::Quit => resolver.quit(),
                Action::Undo => self.state.history.undo(*count, &mut self.state.solve),
                Action::Redo => self.state.history.redo(*count, &mut self.state.solve),
                Action::NextMode(mode) => {
                    let selection = &mut self.state.render.selection;

                    match mode {
                        EventMode::Visual(kind) => {
                            selection.set_kind(*kind);
                        }
                        _ => {
                            if let Some(start) = selection.start() {
                                self.state.render.cursor = start;
                            }

                            selection.reset();
                        }
                    }
                }

                // Focus change actions
                action if action.is_focus() => {
                    return self
                        .state
                        .focus
                        .handle_base_action(action.clone(), &mut ctx.mode);
                }
                _ => {}
            }
        }

        match self.state.focus.get() {
            Focus::Crossword => {
                self.crossword
                    .handle_command(command, resolver, ctx, &mut self.state)
            }
            Focus::Clues => self
                .clues
                .handle_command(command, resolver, ctx, &mut self.state),
            Focus::Footer => self
                .crossword
                .handle_command(command, resolver, ctx, &mut self.state),
        }
    }

    fn on_pause(&mut self, _ctx: &mut AppContext<AppState>) {
        self.state.solve.timer.pause();
    }

    fn on_resume(&mut self, _ctx: &mut AppContext<AppState>) {
        self.state.solve.timer.start();
    }
}
