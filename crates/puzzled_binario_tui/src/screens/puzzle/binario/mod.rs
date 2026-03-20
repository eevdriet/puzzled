mod render;

pub(crate) use render::*;

use puzzled_tui::{GridWidget, RenderSize, Widget as AppWidget};
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
}
