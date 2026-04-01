mod render;

pub(crate) use render::*;

use crossterm::event::{KeyCode, MouseButton};
use puzzled_binario::{BinarioState, Bit};
use puzzled_core::{Entry, Grid, Side, Solve};
use puzzled_tui::{
    Action, AppCommand, AppContext, AppResolver, Command, EventMode, HandleBaseAction,
    SidedGridWidget, SidedGridWidgetState, Widget as AppWidget, handle_grid_command,
};
use ratatui::prelude::{Buffer, Rect, Size};

use crate::{BinarioApp, PuzzleScreenState};

pub struct BinarioWidget;

impl AppWidget<BinarioApp> for BinarioWidget {
    type State = PuzzleScreenState;

    fn render(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        ctx: &mut AppContext<BinarioApp>,
        state: &mut Self::State,
    ) {
        let mut cell_state = ();
        let mut edge_state = ();

        let (grid, row_bits, col_bits) = self.grid_components(&state.solve);
        let mut grid = SidedGridWidget::from_grid(&grid)
            .with_left(row_bits)
            .with_top(col_bits);

        let mut grid_state =
            SidedGridWidgetState::new(&mut state.render, &mut cell_state, &mut edge_state);
        grid.render(area, buf, ctx, &mut grid_state);
    }

    fn render_size(
        &self,
        area: Rect,
        ctx: &AppContext<BinarioApp>,
        state: &mut Self::State,
    ) -> Size {
        let mut cell_state = ();
        let mut edge_state = ();

        let (grid, row_bits, col_bits) = self.grid_components(&state.solve);
        let grid = SidedGridWidget::from_grid(&grid)
            .with_left(row_bits)
            .with_top(col_bits);
        let mut grid_state =
            SidedGridWidgetState::new(&mut state.render, &mut cell_state, &mut edge_state);

        grid.render_size(area, ctx, &mut grid_state)
    }

    fn on_command(
        &mut self,
        command: AppCommand<BinarioApp>,
        resolver: AppResolver<BinarioApp>,
        _ctx: &mut AppContext<BinarioApp>,
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
                    Some(state.bit),
                ) {
                    Some(action) => {
                        state.history.execute(action, &mut state.solve);
                        true
                    }
                    _ => false,
                }
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

type Components<'a> = (Grid<Entry<RenderBit<'a>>>, &'a Vec<bool>, &'a Vec<bool>);

impl BinarioWidget {
    fn grid_components<'a>(&'a self, solve: &'a BinarioState) -> Components<'a> {
        // Create grid widget
        let grid = solve.state.map_entries(|bit| RenderBit { bit });
        let row_bits = solve.valid.get_side(Side::Left).expect("Should be defined");
        let col_bits = solve.valid.get_side(Side::Top).expect("Should be defined");

        (grid, row_bits, col_bits)
    }
}
