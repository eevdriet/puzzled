mod render;

use crossterm::event::{KeyCode, MouseButton};
use puzzled_binario::Bit;
use puzzled_core::{Entry, Solve};
pub(crate) use render::*;

use puzzled_tui::{
    Action, AppCommand, AppContext, AppResolver, Command, EventMode, HandleBaseAction, RenderSize,
    SidedGridWidget, Widget as AppWidget, handle_grid_command,
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
        let PuzzleScreenState { solve, render, .. } = state;

        let render_c = render.clone();

        let grid = solve.state.to_merged();

        let grid = grid.map_ref(|solution_entry| {
            let cell = solution_entry.get().map(|bit| RenderBit { bit });

            Entry::new_with_style(cell, solution_entry.entry.style())
        });

        let mut sided_grid_widget =
            SidedGridWidget::new(&grid, &solve.valid.sides, &render_c.grid, &render_c.sides);

        AppWidget::<BinarioApp>::render(&mut sided_grid_widget, area, buf, ctx, &mut state.render);
    }

    fn render_size(&self, area: Rect, _ctx: &AppContext<BinarioApp>, state: &Self::State) -> Size {
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
        _ctx: &mut AppContext<BinarioApp>,
        state: &mut Self::State,
    ) -> bool {
        match command {
            command @ (Command::Operator(..) | Command::Motion { .. }) => {
                let mut custom_state = ();

                handle_grid_command(
                    command,
                    resolver,
                    &mut state.render.grid,
                    &mut state.solve.state,
                    &mut custom_state,
                    &mut state.history,
                )
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
