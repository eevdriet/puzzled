mod description;
mod footer;
mod nonogram;
mod rules;
mod state;

pub use footer::*;
pub use nonogram::*;
pub use rules::*;
pub use state::*;

use puzzled_nonogram::{Nonogram, NonogramState};
use puzzled_tui::{
    Action, ActionHistory, AppCommand, AppContext, AppResolver, Command, EdgeRender, EventMode,
    FocusManager, HandleMode, Keys, KeysListPopup, KeysTablePopup, KeysTablePopupState, Popup,
    Screen, SidedGridRenderState, Widget as AppWidget,
};
use ratatui::{
    layout::{Constraint, Flex, Layout, Size},
    prelude::{Buffer, Rect},
    widgets::ListState,
};

use crate::NonogramApp;

pub struct PuzzleScreen {
    pub state: PuzzleScreenState,

    // Widgets
    nonogram: NonogramWidget,

    // Popups
    pause: KeysListPopup<NonogramApp>,
}

impl PuzzleScreen {
    pub fn new(puzzle: Nonogram, solve: NonogramState, render: SidedGridRenderState) -> Self {
        let (fill, _) = puzzle
            .colors()
            .first_key_value()
            .expect("At least one color should be present");
        let fill = *fill;

        let pause_keys = Keys::default()
            .action(Action::Quit, &())
            .action(Action::Cancel, &());

        let mut pause_state = ListState::default();
        pause_state.select_first();

        let state = PuzzleScreenState {
            puzzle,
            solve,
            render,
            fill,
            focus: FocusManager::default(),
            popup: None,
            history: ActionHistory::default(),
            pause_keys,
            pause_state,
            help_state: KeysTablePopupState::default(),
        };

        Self {
            state,
            nonogram: NonogramWidget,
            pause: KeysListPopup::new("Paused puzzle"),
        }
    }
}

impl Screen<NonogramApp> for PuzzleScreen {
    fn render(&mut self, root: Rect, buf: &mut Buffer, ctx: &mut AppContext<NonogramApp>) {
        let PuzzleScreen {
            state, nonogram, ..
        } = self;

        // Set the maximum display length for the rules
        state.render.sides.top.max_len = Some(4 * root.height / 10);
        state.render.sides.left.max_len = Some(4 * root.width / 10);

        let puzzle_size = nonogram.render_size(root, ctx, state);

        // Determine how many lines are needed to display the current rule
        let rules_size = Size::new(
            puzzle_size.width.min(root.width),
            puzzle_size.height.min(root.height),
        );
        let rules_size = state.max_rule_size(rules_size);

        let left_width = nonogram.left_width(root, ctx, state);
        let puzzle_width = puzzle_size.width.min(root.width - rules_size.width - 1);
        let puzzle_height = puzzle_size
            .height
            .min(root.height - rules_size.height - FooterWidget::HEIGHT);

        // Vertical
        let [puzzle_area, row_rule_area, footer_area] = Layout::vertical([
            Constraint::Length(puzzle_height),
            Constraint::Length(rules_size.height),
            Constraint::Length(FooterWidget::HEIGHT),
        ])
        .flex(Flex::Center)
        .areas(root);

        // Horizontal
        let center_offset_width = root
            .width
            .saturating_sub(puzzle_width)
            .saturating_sub(left_width)
            / 2;

        let [_, puzzle_area, _, col_rule_area] = Layout::horizontal([
            Constraint::Length(center_offset_width),
            Constraint::Length(puzzle_width),
            Constraint::Length(1),
            Constraint::Length(rules_size.width),
        ])
        .areas(puzzle_area);

        tracing::trace!("Layout");
        tracing::trace!("\tRoot: {root:?}");
        tracing::trace!("\tPuzzle size: {puzzle_size:?}");
        tracing::trace!("\tRules size: {rules_size:?}");
        tracing::trace!("\tFooter height: {:?}", FooterWidget::HEIGHT);
        tracing::trace!("");
        tracing::trace!("\tPuzzle area: {puzzle_area:?}");
        tracing::trace!("\tRow rule area: {row_rule_area:?}");
        tracing::trace!("\tCol rule area: {col_rule_area:?}");
        tracing::trace!("\tFooter area: {footer_area:?}");

        // Render
        // - Puzzle
        nonogram.render(puzzle_area, buf, ctx, state);

        // - Rules
        let rules = state.puzzle.rules();
        let (row, col) = state.render.grid.cursor.lines();
        let row_rule = RenderRule {
            rule: rules.get(&row).expect("Cursor should get a valid rule"),
        };
        let col_rule = RenderRule {
            rule: rules.get(&col).expect("Cursor should get a valid rule"),
        };

        let rule_state = RenderRuleState {
            is_active_rule: true,
            colors: state.puzzle.colors(),
        };

        row_rule.render_row(
            row.line(),
            row_rule_area,
            buf,
            ctx,
            &state.render,
            &rule_state,
        );
        col_rule.render_col(
            col.line(),
            col_rule_area,
            buf,
            ctx,
            &state.render,
            &rule_state,
        );

        // Footer
        let mut footer_state = FooterState {
            colors: self.state.puzzle.colors(),
            fill: &self.state.fill,
        };
        let mut footer = FooterWidget::new();
        footer.render(footer_area, buf, ctx, &mut footer_state);

        // Popups
        if let Some(popup) = self.state.popup {
            match popup {
                PuzzlePopup::Pause => {
                    self.pause
                        .render_popup(root, buf, ctx, &mut self.state.pause_state);
                }
                PuzzlePopup::Help => {
                    let keys = Keys::new().all(&self.state);
                    let mut help = KeysTablePopup::new(&keys);

                    help.render_popup(root, buf, ctx, &mut self.state.help_state);
                }
            }
        }
    }

    fn on_command(
        &mut self,
        command: AppCommand<NonogramApp>,
        resolver: AppResolver<NonogramApp>,
        ctx: &mut AppContext<NonogramApp>,
    ) -> bool {
        // Handle popup commands
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

                    return help.on_command(command, resolver, ctx, &mut self.state.help_state);
                }
            }
        }

        let mut handled_action = false;

        if let Command::Action { count, action } = &command {
            handled_action = true;

            match action {
                // Lifetime actions
                Action::Cancel => {
                    self.state.popup = Some(PuzzlePopup::Pause);
                    resolver.open_popup();
                }
                Action::ShowHelp => {
                    self.state.popup = Some(PuzzlePopup::Help);
                    resolver.open_popup();
                }
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

    fn on_popup_close(&mut self, _ctx: &mut AppContext<NonogramApp>) {
        self.state.popup = None;
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
