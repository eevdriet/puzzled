mod render;

pub(crate) use render::*;

use puzzled_core::Solve;
use puzzled_tui::{
    Action, AppContext, Command, GridWidget, RenderSize, Widget as AppWidget, handle_grid_command,
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
        let render_c = state.render.clone();

        // Create grid widget
        let grid = state.solve.state.map_entries(|fill| RenderFill { fill });

        let cell_state = RenderFillState {
            colors: state.puzzle.colors(),
            render: &render_c,
        };

        let mut grid_widget = GridWidget::new(&grid, &cell_state);

        // Render
        let area = self.render_area(root, ctx, state);
        grid_widget.render(area, buf, ctx, &mut state.render);
    }

    fn render_size(&self, area: Rect, _ctx: &AppContext<NonogramApp>, state: &Self::State) -> Size {
        state.puzzle.fills().render_size(area, &state.render)
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
                    &mut state.render,
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
                    match state.render.to_grid(pos) {
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

                _ => false,
            },
            _ => false,
        }
    }
}
