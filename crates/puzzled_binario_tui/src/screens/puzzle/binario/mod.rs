mod render;

use puzzled_binario::Bit;
use puzzled_core::{Direction, Solve};
pub(crate) use render::*;

use puzzled_tui::{
    Action, AppCommand, AppResolver, AsCore, Command, EventMode, GridWidget, HandleBaseMotion,
    HandleOperator, Operator, RenderSize, Widget as AppWidget,
};
use ratatui::prelude::{Buffer, Rect, Size, StatefulWidget};
use tui_scrollview::ScrollView;

use crate::{BinarioApp, PuzzleScreenState};

pub struct BinarioWidget;

impl AppWidget<BinarioApp> for BinarioWidget {
    type State = PuzzleScreenState;

    fn render(&mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let PuzzleScreenState { solve, render, .. } = state;

        let cell_state = RenderBitState {
            cursor: render.cursor,
            opts: render.options,
        };

        let grid = solve.entries.map_ref(RenderBit);
        let grid_size = grid.render_size(&render.options);
        let grid_widget = GridWidget::new(&grid, &cell_state);

        let mut scroll_view = ScrollView::new(grid_size);

        scroll_view.render_stateful_widget(grid_widget, Rect::from(grid_size), render);
        scroll_view.render(area, buf, &mut render.scroll);
    }

    fn render_size(&self, state: &Self::State) -> Size {
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
            Command::Operator(op) => {
                if state.render.mode.is_visual() {
                    let size = state.puzzle.cells().size();
                    let positions = state
                        .render
                        .selection
                        .range(size)
                        .positions()
                        .map(|pos| pos.as_core());

                    state
                        .solve
                        .0
                        .handle_operator(op, positions, &mut state.history);

                    let mode = match op {
                        Operator::Change => EventMode::Insert,
                        _ => EventMode::Normal,
                    };
                    resolver.set_mode(mode);
                } else if !op.requires_motion() {
                    let positions = vec![state.render.cursor];

                    state
                        .solve
                        .0
                        .handle_operator(op, positions, &mut state.history);
                } else {
                    return false;
                }
            }
            Command::Motion { count, motion, op } if state.render.mode.is_visual() => {
                assert!(op.is_none());

                let cells = state.puzzle.cells();
                let positions = cells.handle_base_motion(count, motion, &mut state.render);

                if let Some(end) = positions.into_iter().last() {
                    state.render.selection.update(end);
                }
            }
            Command::Motion { count, motion, op } => {
                let cells = state.puzzle.cells();
                let positions = cells.handle_base_motion(count, motion, &mut state.render);

                if let Some(op) = op {
                    state
                        .solve
                        .0
                        .handle_operator(op, positions, &mut state.history);
                }
            }
            Command::Action { action, .. } => {
                let pos = state.render.cursor;
                let dir = match state.render.direction {
                    Direction::Left | Direction::Right => Direction::Right,
                    Direction::Up | Direction::Down => Direction::Down,
                };

                match action {
                    Action::Literal(letter @ '0') | Action::Literal(letter @ '1') => {
                        let bit = Bit::try_from(letter as u8 - b'0').expect("Verified bit input");
                        state.solve.enter(&pos, bit);

                        if let Some(next) = pos + dir
                            && state.puzzle.cells().get(next).is_some()
                        {
                            state.render.cursor = next;
                        }
                    }

                    _ => return false,
                }
            }
            _ => return false,
        }

        true
    }
}
