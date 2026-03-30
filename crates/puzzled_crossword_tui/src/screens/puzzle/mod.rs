mod clues;
mod crossword;
mod description;
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
    FocusManager, GridRenderState, HandleCommand, HandleMode, Keys, KeysListPopup, KeysTablePopup,
    KeysTablePopupState, Popup, Screen, TrieEntry, Widget,
};

use crate::CrosswordApp;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Focus {
    #[default]
    Crossword,

    Clues,

    Footer,
}

#[derive(Debug, Clone, Copy)]
pub enum PuzzlePopup {
    Pause,
    Help,
}

pub struct PuzzleScreen {
    state: PuzzleScreenState,

    // Widgets
    crossword: CrosswordWidget,
    clues: CluesWidget,
    footer: FooterWidget,

    // Popups
    pause: KeysListPopup<CrosswordApp>,
}

impl PuzzleScreen {
    pub fn new(
        puzzle: Crossword,
        solve_state: CrosswordState,
        render_state: GridRenderState,
    ) -> Self {
        let mut focus = FocusManager::default();

        focus.link_right(Focus::Crossword, &[Focus::Clues]);
        focus.link_below(Focus::Footer, &[Focus::Clues]);

        let list = ListState::default().with_selected(Some(0));

        let pause_keys = Keys::default()
            .action(Action::Quit, &())
            .action(Action::Cancel, &());
        let mut pause_state = ListState::default();
        pause_state.select_first();

        let pause = KeysListPopup::new("Paused puzzle".to_string());

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
            popup: None,
            pause_keys,
            pause_state,
            help_state: KeysTablePopupState::default(),
        };

        Self {
            state,
            crossword: CrosswordWidget,
            clues: CluesWidget::default(),
            footer: FooterWidget,
            pause,
        }
    }
}

impl Screen<CrosswordApp> for PuzzleScreen {
    fn render(&mut self, root: Rect, buf: &mut Buffer, ctx: &mut AppContext<CrosswordApp>) {
        // Compute sizes
        let gap = 2;

        let crossword_size = self.crossword.render_size(root, ctx, &self.state);
        let clues_size = self.clues.render_size(root, ctx, &self.state);

        let entry = TrieEntry::Action(Action::Cancel);
        let pause_key = ctx.keys.get_merged_str(&entry).unwrap_or_default();

        let mut footer_state = FooterState {
            mode: self.state.render.mode,
            timer: self.state.solve.timer,
            pause_key,
        };
        let footer_size = self.footer.render_size(root, ctx, &footer_state);

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

        let clues_height = clues_size.height.min(right.height - footer_size.height);
        let [clues, footer, _] = Layout::vertical(vec![
            Constraint::Length(clues_height),
            Constraint::Length(footer_size.height),
            Constraint::Min(0),
        ])
        .areas(right);

        tracing::trace!("Root: {root:?}");
        tracing::trace!("Crossword: {crossword:?}");
        tracing::trace!("Right: {right:?}");
        tracing::trace!("Clues: {clues:?}");
        tracing::trace!("Footer: {footer:?}");

        // Render widgets
        self.crossword.render(crossword, buf, ctx, &mut self.state);
        self.clues.render(clues, buf, ctx, &mut self.state);
        self.footer.render(footer, buf, ctx, &mut footer_state);

        // Render popups
        if let Some(popup) = self.state.popup {
            match popup {
                PuzzlePopup::Pause => {
                    self.pause
                        .render_popup(area, buf, ctx, &mut self.state.pause_state);
                }
                PuzzlePopup::Help => {
                    let keys = Keys::new().all(&self.state);
                    let mut help = KeysTablePopup::new(&keys);
                    help.render_popup(area, buf, ctx, &mut self.state.help_state);
                }
            }
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
        if let Some(popup) = self.state.popup {
            match popup {
                PuzzlePopup::Pause => {
                    return self.pause.on_popup_command(
                        command,
                        resolver,
                        ctx,
                        &mut self.state.pause_state,
                    );
                }
                PuzzlePopup::Help => {
                    let keys = Keys::new().all(&self.state);
                    let mut help = KeysTablePopup::new(&keys);

                    return help.on_popup_command(
                        command,
                        resolver,
                        ctx,
                        &mut self.state.help_state,
                    );
                }
            }
        }

        let mut handled_action = false;

        if let Command::Action { action, count } = &command {
            handled_action = true;

            match action {
                // Lifetime actions
                Action::Quit => {
                    resolver.prev_screen();
                }
                Action::Cancel => {
                    self.state.popup = Some(PuzzlePopup::Pause);
                    resolver.open_popup();
                }
                Action::ShowHelp => {
                    self.state.popup = Some(PuzzlePopup::Help);
                    resolver.open_popup();
                }
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
                Focus::Crossword => {
                    self.crossword
                        .on_command(command, resolver, ctx, &mut self.state)
                }
                Focus::Clues => self
                    .clues
                    .on_command(command, resolver, ctx, &mut self.state),
                Focus::Footer => self
                    .crossword
                    .on_command(command, resolver, ctx, &mut self.state),
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
            Focus::Footer => self.footer.override_mode(),
        }
    }

    fn on_popup_close(&mut self, _ctx: &mut AppContext<CrosswordApp>) {
        self.state.popup = None;
    }

    fn on_enter(&mut self, _ctx: &mut AppContext<CrosswordApp>) {
        self.state.solve.timer.start();
    }

    fn on_pause(&mut self, _ctx: &mut AppContext<CrosswordApp>) {
        self.state.solve.timer.pause();
    }

    fn on_resume(&mut self, _ctx: &mut AppContext<CrosswordApp>) {
        self.state.solve.timer.start();
    }
}
