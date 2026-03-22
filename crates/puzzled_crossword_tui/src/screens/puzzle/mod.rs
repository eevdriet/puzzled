mod clues;
mod crossword;
mod footer;
mod hello;
mod state;

pub use clues::*;
pub use crossword::*;
pub use footer::*;
pub use hello::*;
pub use state::*;

use ratatui::{
    layout::{Constraint, Layout},
    prelude::{Buffer, Rect},
    widgets::ListState,
};

use puzzled_crossword::{ClueDirection, Crossword, CrosswordState};
use puzzled_tui::{
    Action, ActionBehavior, ActionHistory, AppCommand, AppContext, AppResolver, Command, EventMode,
    FocusManager, GridRenderState, HandleCommand, HandleMode, KeyMap, KeysPopup, KeysPopupState,
    Popup, Screen, Widget,
};

use crate::CrosswordApp;

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

    // Popups
    popup: bool,
    keys: KeysPopup<CrosswordApp>,
}

impl PuzzleScreen {
    pub fn new(
        puzzle: Crossword,
        solve_state: CrosswordState,
        render_state: GridRenderState,
        keys: KeyMap<CrosswordApp>,
    ) -> Self {
        let mut focus = FocusManager::default();

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
            is_paused: false,
        };

        let keys = KeysPopup::new(keys).all_actions(&());

        Self {
            state,
            popup: false,
            crossword: CrosswordWidget,
            clues: CluesWidget::default(),
            keys,
        }
    }
}

impl Screen<CrosswordApp> for PuzzleScreen {
    fn render(&mut self, root: Rect, buf: &mut Buffer, _ctx: &mut AppContext<CrosswordApp>) {
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

        let footer_height = 5;
        let clues_height = clues_size.height.min(right.height - footer_height);
        let [clues, footer, _] = Layout::vertical(vec![
            Constraint::Length(clues_height),
            Constraint::Length(footer_height),
            Constraint::Min(0),
        ])
        .areas(right);

        tracing::debug!("Root: {root:?}");
        tracing::debug!("Crossword: {crossword:?}");
        tracing::debug!("Right: {right:?}");
        tracing::debug!("Clues: {clues:?}");
        tracing::debug!("Footer: {footer:?}");

        // Render
        self.crossword.render(crossword, buf, &mut self.state);
        self.clues.render(clues, buf, &mut self.state);

        let mut footer_state = FooterState {
            mode: self.state.render.mode,
            timer: self.state.solve.timer,
        };

        FooterWidget.render(footer, buf, &mut footer_state);

        if self.popup {
            let mut state = KeysPopupState::default();
            self.keys.render_popup(area, buf, &mut state);
        }
    }

    fn on_tick(&self, _ctx: &AppContext<CrosswordApp>) -> bool {
        true
    }

    fn on_command(
        &mut self,
        command: AppCommand<CrosswordApp>,
        resolver: AppResolver<CrosswordApp>,
        ctx: &mut AppContext<CrosswordApp>,
    ) -> bool {
        if self.popup {
            let mut state = KeysPopupState::default();
            return self.keys.on_popup_command(command, resolver, &mut state);
        }

        let mut handled_action = false;

        if let Command::Action { action, count } = &command {
            handled_action = true;

            match action {
                // Lifetime actions
                Action::Cancel => resolver.prev_screen(),
                Action::ShowHelp => resolver.open_popup(),
                Action::Quit => resolver.quit(),
                Action::Undo => self.state.history.undo(*count, &mut self.state.solve),
                Action::Redo => self.state.history.redo(*count, &mut self.state.solve),

                // Focus change actions
                action if action.is_focus() => {
                    return self
                        .state
                        .focus
                        .handle_command(command, resolver, ctx, &mut ());
                }
                _ => {
                    handled_action = false;
                }
            }
        }

        handled_action
            || match self.state.focus.get() {
                Focus::Crossword => self
                    .crossword
                    .on_command(command, resolver, &mut self.state),
                Focus::Clues => self.clues.on_command(command, resolver, &mut self.state),
                Focus::Footer => self
                    .crossword
                    .on_command(command, resolver, &mut self.state),
            }
    }

    fn on_mode(
        &mut self,
        mode: EventMode,
        resolver: AppResolver<CrosswordApp>,
        ctx: &mut AppContext<CrosswordApp>,
    ) -> bool {
        let solutions = &mut self.state.solve.solutions;
        solutions.handle_mode(mode, resolver, ctx, &mut self.state.render)
    }

    fn override_mode(&self) -> Option<EventMode> {
        match self.state.focus.get() {
            Focus::Clues => self.clues.override_mode(),
            Focus::Crossword => self.crossword.override_mode(),
            _ => None,
        }
    }

    fn on_popup_open(&mut self, _ctx: &mut AppContext<CrosswordApp>) {
        self.popup = true;
    }

    fn on_popup_close(&mut self, _ctx: &mut AppContext<CrosswordApp>) {
        self.popup = false;
    }

    fn on_enter(&mut self, _ctx: &mut AppContext<CrosswordApp>) {
        self.state.solve.timer.start();
    }

    fn on_pause(&mut self, _ctx: &mut AppContext<CrosswordApp>) {
        self.state.solve.timer.pause();
        self.state.is_paused = true;
    }

    fn on_resume(&mut self, _ctx: &mut AppContext<CrosswordApp>) {
        self.state.solve.timer.start();
        self.state.is_paused = false;
    }
}
