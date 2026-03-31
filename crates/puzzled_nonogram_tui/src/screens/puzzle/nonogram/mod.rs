mod render;

use std::collections::HashMap;

use puzzled_nonogram::{Nonogram, NonogramState};
pub(crate) use render::*;

use puzzled_core::{Entry, Grid, Side, Solve};
use puzzled_tui::{
    Action, AppContext, Command, GridWidgetState, HandleBaseAction, SidedGridWidget,
    SidedGridWidgetState, Widget as AppWidget, handle_grid_command,
};
use ratatui::prelude::{Buffer, Rect, Size};

use crate::{NonogramApp, PuzzleScreenState, RenderRule, RenderRuleState};

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
        let area = self.render_area(root, ctx, state);

        // Create the puzzle widget
        let PuzzleScreenState {
            puzzle,
            solve,
            render,
            ..
        } = state;

        let (grid, sides) = self.grid_components(puzzle, solve);

        let mut grid_widget = SidedGridWidget::new(&grid, &sides);
        let mut grid_widget_state = SidedGridWidgetState {
            grid: GridWidgetState {
                render: &mut render.grid,
                cell_state: puzzle.colors(),
            },
            sides: &mut render.sides,
            edge_state: RenderRuleState {
                colors: puzzle.colors(),
                is_active_rule: false,
            },
        };

        // Render
        grid_widget.render(area, buf, ctx, &mut grid_widget_state);
    }

    fn render_size(
        &self,
        area: Rect,
        ctx: &AppContext<NonogramApp>,
        state: &mut Self::State,
    ) -> Size {
        // Sided grid size
        let (grid, sides) = self.grid_components(&state.puzzle, &state.solve);
        let sided_grid_widget = SidedGridWidget::new(&grid, &sides);
        let mut grid_widget_state = SidedGridWidgetState {
            grid: GridWidgetState {
                render: &mut state.render.grid,
                cell_state: state.puzzle.colors(),
            },
            sides: &mut state.render.sides,
            edge_state: RenderRuleState {
                colors: state.puzzle.colors(),
                is_active_rule: false,
            },
        };

        sided_grid_widget.render_size(area, ctx, &mut grid_widget_state)
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
);

impl NonogramWidget {
    fn grid_components<'a>(
        &'a self,
        puzzle: &'a Nonogram,
        solve: &'a NonogramState,
    ) -> Components<'a> {
        // Create grid widget
        let grid = solve.state.map_entries(|fill| RenderFill { fill });

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

        (grid, sides)
    }
}
