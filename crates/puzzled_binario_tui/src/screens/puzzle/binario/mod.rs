mod render;

use crossterm::event::{KeyCode, MouseButton};
use puzzled_binario::Bit;
use puzzled_core::Solve;
pub(crate) use render::*;

use puzzled_tui::{
    Action, AppCommand, AppResolver, Command, EventMode, GridWidget, HandleBaseAction,
    HandleMotion, HandleOperator, RenderSize, Widget as AppWidget, handle_grid_operator,
};
use ratatui::prelude::{Buffer, Rect, Size, StatefulWidget};
use tui_scrollview::ScrollView;

use crate::{BinarioApp, PuzzleScreenState};

pub struct BinarioWidget;

impl AppWidget<BinarioApp> for BinarioWidget {
    type State = PuzzleScreenState;

    fn render(&mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let PuzzleScreenState { solve, render, .. } = state;

        render.viewport = area;

        let render_c = render.clone();
        let cell_state = RenderBitState { render: &render_c };

        // let grid = solve.entries.map_ref(RenderBit);

        let grid = solve.to_merged().map(RenderBit);
        let grid_size = grid.render_size(&render.options);
        let grid_widget = GridWidget::new(&grid, &cell_state);

        let mut scroll_view = ScrollView::new(grid_size);

        scroll_view.render_stateful_widget(grid_widget, Rect::from(grid_size), render);
        scroll_view.render(area, buf, &mut render.scroll);
    }

    fn render_size(&self, _area: Rect, state: &Self::State) -> Size {
        let mut size = state.puzzle.cells().render_size(&state.render.options);

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
                &state.render,
                &mut state.solve.0,
                &mut state.history,
            ),
            Command::Motion { count, motion, op } => {
                let cells = state.puzzle.cells();
                let mut custom_state = ();
                let positions =
                    cells.handle_motion(count, motion, &mut state.render, &mut custom_state);

                if let Some(op) = op {
                    state
                        .solve
                        .0
                        .handle_operator(op, positions, &mut state.history);
                }
                true
            }
            Command::Action { action, .. } => {
                let pos = state.render.cursor;

                match action {
                    Action::Click(button) => {
                        let bit = match button {
                            MouseButton::Left => Bit::Zero,
                            _ => Bit::One,
                        };
                        let entry = &state.solve.entries[pos];

                        match entry.entry() {
                            None => state.solve.enter(&pos, bit),
                            Some(other) if *other == bit => state.solve.enter(&pos, !bit),
                            _ => state.solve.clear(&pos),
                        };

                        true
                    }
                    Action::Literal(KeyCode::Char(' ')) => {
                        let bit = &state.solve.entries[pos];

                        match bit.entry() {
                            None => state.solve.enter(&pos, Bit::Zero),
                            Some(Bit::Zero) => state.solve.enter(&pos, Bit::One),
                            Some(Bit::One) => state.solve.clear(&pos),
                        };

                        true
                    }

                    _ => state
                        .solve
                        .0
                        .solutions
                        .handle_action(action, &mut state.render, &mut ()),
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
