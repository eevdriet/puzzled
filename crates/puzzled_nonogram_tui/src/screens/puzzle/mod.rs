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
    FocusManager, HandleMode, Screen, SidedGridRenderState, Widget as AppWidget,
};
use ratatui::{
    layout::{Constraint, Layout, Size},
    prelude::{Buffer, Rect},
};

use crate::NonogramApp;

pub struct PuzzleScreen {
    pub state: PuzzleScreenState,

    // Widgets
    nonogram: NonogramWidget,
}

impl PuzzleScreen {
    pub fn new(puzzle: Nonogram, solve: NonogramState, render: SidedGridRenderState) -> Self {
        let (fill, _) = puzzle
            .colors()
            .first_key_value()
            .expect("At least one color should be present");
        let fill = *fill;

        let state = PuzzleScreenState {
            puzzle,
            solve,
            render,
            fill,
            focus: FocusManager::default(),
            history: ActionHistory::default(),
        };

        Self {
            state,
            nonogram: NonogramWidget,
        }
    }
}

impl Screen<NonogramApp> for PuzzleScreen {
    fn render(&mut self, root: Rect, buf: &mut Buffer, ctx: &mut AppContext<NonogramApp>) {
        let PuzzleScreen { state, nonogram } = self;

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

        let puzzle_width = puzzle_size.width.min(root.width - rules_size.width - 1);
        let puzzle_height = puzzle_size
            .height
            .min(root.height - rules_size.height - FooterWidget::HEIGHT);

        let [puzzle_area, row_rule_area, footer_area] = Layout::vertical([
            Constraint::Length(puzzle_height),
            Constraint::Length(rules_size.height),
            Constraint::Length(FooterWidget::HEIGHT),
        ])
        .areas(root);

        let [puzzle_area, _, col_rule_area] = Layout::horizontal([
            Constraint::Length(puzzle_width),
            Constraint::Length(1),
            Constraint::Length(rules_size.width),
        ])
        .areas(puzzle_area);

        tracing::info!("Layout");
        tracing::info!("\tRoot: {root:?}");
        tracing::info!("\tPuzzle size: {puzzle_size:?}");
        tracing::info!("\tRules size: {rules_size:?}");
        tracing::info!("\tFooter height: {:?}", FooterWidget::HEIGHT);
        tracing::info!("");
        tracing::info!("\tPuzzle area: {puzzle_area:?}");
        tracing::info!("\tRow rule area: {row_rule_area:?}");
        tracing::info!("\tCol rule area: {col_rule_area:?}");
        tracing::info!("\tFooter area: {footer_area:?}");

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
    }

    fn on_tick(&self, _ctx: &AppContext<NonogramApp>) -> bool {
        true
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
