mod render;

use std::collections::HashMap;

pub(crate) use render::*;

use puzzled_core::{Direction, Solve};
use puzzled_tui::{
    Action, AppContext, Command, HandleBaseAction, RenderSize, SidedGridWidget,
    Widget as AppWidget, handle_grid_command,
};
use ratatui::{
    layout::Margin,
    prelude::{Buffer, Rect, Size},
    widgets::{Block, Widget},
};

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
        let PuzzleScreenState { solve, render, .. } = state;

        let render_c = render.clone();

        // Create grid widget
        let grid = solve.state.map_entries(|fill| RenderFill { fill });

        let cell_state = RenderFillState {
            colors: state.puzzle.colors(),
            render: &render_c.grid,
        };
        let line_state = RenderRuleState {
            colors: state.puzzle.colors(),
            render: &render_c.sides,
        };

        let rules = state.puzzle.rules();
        let sides: HashMap<Direction, Vec<RenderRule>> = HashMap::from([
            (
                Direction::Left,
                rules
                    .iter_rows()
                    .map(|(_, rule)| RenderRule { rule })
                    .collect(),
            ),
            (
                Direction::Up,
                rules
                    .iter_cols()
                    .map(|(_, rule)| RenderRule { rule })
                    .collect(),
            ),
        ]);

        let mut sided_grid_widget = SidedGridWidget::new(&grid, &sides, &cell_state, &line_state);

        // Render
        let title = format!("Nonogram: {}x{}", state.puzzle.rows(), state.puzzle.cols());

        sided_grid_widget.render(root, buf, ctx, &mut state.render);
    }

    fn render_size(&self, area: Rect, _ctx: &AppContext<NonogramApp>, state: &Self::State) -> Size {
        let mut size = state.puzzle.fills().render_size(area, &state.render.grid);

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
