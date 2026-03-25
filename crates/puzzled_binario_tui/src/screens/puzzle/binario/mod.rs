mod render;

use crossterm::event::{KeyCode, MouseButton};
use puzzled_binario::Bit;
use puzzled_core::Solve;
pub(crate) use render::*;

use puzzled_tui::{
    Action, AppCommand, AppResolver, Command, EventMode, HandleBaseAction, HandleMotion,
    HandleOperator, RenderSize, SidedGridWidget, Widget as AppWidget, handle_grid_operator,
};
use ratatui::prelude::{Buffer, Rect, Size};

use crate::{BinarioApp, PuzzleScreenState};

pub struct BinarioWidget;

impl AppWidget<BinarioApp> for BinarioWidget {
    type State = PuzzleScreenState;

    fn render(&mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let PuzzleScreenState { solve, render, .. } = state;

        let render_c = render.clone();
        let cell_state = RenderBitState {
            render: &render_c.grid,
        };
        let edge_state = RenderEdgeState;

        let grid = solve.state.to_merged();
        let grid = grid
            .join_ref(&solve.validity.grid, |solution_entry, validity| RenderBit {
                solution_entry,
                validity: *validity,
            })
            .expect("Solve grids have the same size");
        // let grid_widget = GridWidget::new(&grid, &cell_state);
        // grid_widget.render(area, buf, &mut state.render);

        let mut sided_grid_widget =
            SidedGridWidget::new(&grid, &solve.validity.sides, &cell_state, &edge_state);

        AppWidget::<BinarioApp>::render(&mut sided_grid_widget, area, buf, &mut state.render);
    }

    fn render_size(&self, area: Rect, state: &Self::State) -> Size {
        let mut size = state
            .puzzle
            .cells()
            .render_size(area, &state.render.grid.options);

        // Border around puzzle grid
        size.width += 2;
        size.height += 2;

        size
    }

    fn on_command(
        &mut self,
        command: AppCommand<BinarioApp>,
        resolver: AppResolver<BinarioApp>,
        state: &mut Self::State,
    ) -> bool {
        match command {
            Command::Operator(op) => handle_grid_operator(
                op,
                resolver,
                &state.render.grid,
                &mut state.solve.state,
                &mut state.history,
            ),
            Command::Motion { count, motion, op } => {
                let cells = state.puzzle.cells();
                let mut custom_state = ();
                let positions =
                    cells.handle_motion(count, motion, &mut state.render.grid, &mut custom_state);

                if let Some(op) = op {
                    state
                        .solve
                        .state
                        .handle_operator(op, positions, &mut state.history);
                }
                true
            }
            Command::Action { action, .. } => {
                let pos = state.render.grid.cursor;

                match action {
                    Action::Click { button, pos } => {
                        let bit = match button {
                            MouseButton::Left => Bit::Zero,
                            _ => Bit::One,
                        };

                        match state.render.grid.to_grid(pos) {
                            Some(pos) => {
                                let entry = &state.solve.state.entries[pos];

                                match entry.entry() {
                                    None => state.solve.enter(&pos, bit),
                                    Some(other) if *other == bit => state.solve.enter(&pos, !bit),
                                    _ => state.solve.clear(&pos),
                                };

                                true
                            }
                            None => false,
                        }
                    }
                    Action::Literal(KeyCode::Char(' ')) => {
                        let bit = &state.solve.state.entries[pos];

                        match bit.entry() {
                            None => state.solve.enter(&pos, Bit::Zero),
                            Some(Bit::Zero) => state.solve.enter(&pos, Bit::One),
                            Some(Bit::One) => state.solve.clear(&pos),
                        };

                        true
                    }

                    _ => state.solve.state.solutions.handle_action(
                        action,
                        &mut state.render.grid,
                        &mut (),
                    ),
                }
            }
            _ => false,
        }
    }

    fn override_mode(&self) -> Option<EventMode> {
        // Some(EventMode::Normal)
        None
    }
}
