mod render;

use std::collections::HashMap;

use puzzled_nonogram::{Nonogram, NonogramState};
pub(crate) use render::*;

use puzzled_core::{Entry, Grid, Side, Solve};
use puzzled_tui::{
    Action, AppContext, Command, HandleBaseAction, SidedGridRenderState, SidedGridWidget,
    Widget as AppWidget, handle_grid_command,
};
use ratatui::prelude::{Buffer, Rect, Size};

use crate::{NonogramApp, PuzzleScreenState};

pub struct NonogramWidget;

impl AppWidget<NonogramApp> for NonogramWidget {
    type State = PuzzleScreenState;

    fn render(
        &mut self,
        root: Rect,
        buf: &mut Buffer,
        ctx: &mut AppContext<NonogramApp>,
        state: &mut Self::State,
    ) {
        let PuzzleScreenState {
            puzzle,
            solve,
            render,
            ..
        } = state;

        // Set the maximum display length for the rules
        render.sides.top.max_len = Some(4 * root.height / 10);
        render.sides.left.max_len = Some(root.width / 4);

        // Create the puzzle widget
        let render_c = render.clone();
        let (grid, sides, cell_state, line_state) = self.grid_components(puzzle, solve, &render_c);

        let mut sided_grid_widget = SidedGridWidget::new(&grid, &sides, &cell_state, &line_state);

        // Render
        let _title = format!("Nonogram: {}x{}", puzzle.rows(), puzzle.cols());

        sided_grid_widget.render(root, buf, ctx, render);
    }

    fn render_size(&self, area: Rect, ctx: &AppContext<NonogramApp>, state: &Self::State) -> Size {
        // Sided grid size
        let (grid, sides, cell_state, line_state) =
            self.grid_components(&state.puzzle, &state.solve, &state.render);
        let sided_grid_widget = SidedGridWidget::new(&grid, &sides, &cell_state, &line_state);

        let mut size = sided_grid_widget.render_size(area, ctx, &state.render);

        // Border aroudn puzzle grid
        size.width += 2;
        size.height += 2;

        size
    }

    fn on_command(
        &mut self,
        command: puzzled_tui::AppCommand<NonogramApp>,
        resolver: puzzled_tui::AppResolver<NonogramApp>,
        _ctx: &mut AppContext<NonogramApp>,
        state: &mut Self::State,
    ) -> bool {
        match command {
            command @ (Command::Operator(..) | Command::Motion { .. }) => {
                let mut custom_state = ();

                match handle_grid_command(
                    command,
                    resolver,
                    &mut state.render.grid,
                    &mut state.solve.state,
                    &mut custom_state,
                ) {
                    Some(action) => {
                        state.history.execute(action, &mut state.solve);
                        true
                    }
                    _ => false,
                }
            }
            Command::Action { action, .. } => match action {
                Action::Click { pos, .. } => {
                    let fill = state.fill;
                    match state.render.grid.to_grid(pos) {
                        Some(pos) => {
                            let entry = &state.solve.state.entries[pos];

                            match entry.entry() {
                                None => state.solve.enter(&pos, fill),
                                _ => state.solve.clear(&pos),
                            };

                            true
                        }
                        None => false,
                    }
                }
                _ => state.solve.state.solutions.handle_action(
                    action,
                    &mut state.render.grid,
                    &mut (),
                ),
            },
            _ => false,
        }
    }
}

type Components<'a> = (
    Grid<Entry<RenderFill<'a>>>,
    HashMap<Side, Vec<RenderRule<'a>>>,
    RenderFillState<'a>,
    RenderRuleState<'a>,
);

impl NonogramWidget {
    fn grid_components<'a>(
        &'a self,
        puzzle: &'a Nonogram,
        solve: &'a NonogramState,
        render: &'a SidedGridRenderState,
    ) -> Components<'a> {
        // Create grid widget
        let grid = solve.state.map_entries(|fill| RenderFill { fill });

        let cell_state = RenderFillState {
            colors: puzzle.colors(),
            render: &render.grid,
        };
        let line_state = RenderRuleState {
            colors: puzzle.colors(),
            render: &render.sides,
        };

        let rules = puzzle.rules();
        let sides: HashMap<Side, Vec<RenderRule>> = HashMap::from([
            (
                Side::Left,
                rules
                    .iter_rows()
                    .map(|(_, rule)| RenderRule { rule })
                    .collect(),
            ),
            (
                Side::Top,
                rules
                    .iter_cols()
                    .map(|(_, rule)| RenderRule { rule })
                    .collect(),
            ),
        ]);

        (grid, sides, cell_state, line_state)
    }
}
