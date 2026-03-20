use puzzled_crossword::{Crossword, CrosswordState, crossword};
use puzzled_tui::{Action, AppCommand, Command, GridRenderState, GridWidget, Screen};
use puzzled_tui::{AppContext, AppResolver};
use ratatui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};

use crate::{CrosswordApp, RenderSquareSolution, RenderSquareState};

pub struct TitleScreen {
    title: Crossword,
}

impl Default for TitleScreen {
    fn default() -> Self {
        let title = crossword!(
            [ C C C . R R . O O O . S S S . S S S ]
            [ C . . . R R . O . O . . S . . S . . ]
            [ C C C . R . R O O O . S S S . S S S ]
            [ . . . . . . . . . . . . . . . . . . ]
            [ . W . . W . O O O . R R . . D D . . ]
            [ . W . . W . O . O . R R . . D . D . ]
            [ W W W . W . O O O . R . R . D D . . ]
        );
        // let title = crossword!(
        //     [ C C C C C . R R R R . . O O O O O . S S S S S . S S S S S ]
        //     [ C . . . . . R . . R . . O . . . O . S . . . . . S . . . . ]
        //     [ C . . . . . R R R . . . O . . . O . S S S S S . S S S S S ]
        //     [ C . . . . . R . . R . . O . . . O . . . . . S . . . . . S ]
        //     [ C C C C C . R . . . R . O O O O O . S S S S S . S S S S S ]
        //     [ . . . . . . . . . . . . . . . . . . . . . . . . . . . . . ]
        //     [ . . . W . . . W . O O O O O . R R R R . . D D D D . . . . ]
        //     [ . . . W . . . W . O . . . O . R . . R . . D . . . D . . . ]
        //     [ . . . W . W . W . O . . . O . R R R . . . D . . . D . . . ]
        //     [ . . . W . W . W . O . . . O . R . . R . . D . . . D . . . ]
        //     [ . . . W W W W W . O O O O O . R . . . R . D D D D . . . . ]
        //
        // );

        Self { title }
    }
}

impl Screen<CrosswordApp> for TitleScreen {
    fn render(&mut self, root: Rect, buf: &mut Buffer, _ctx: &mut AppContext<CrosswordApp>) {
        let state = CrosswordState::from(&self.title);
        let cell_state = RenderSquareState::new(self.title.squares(), self.title.clues());
        let mut render = GridRenderState::default();
        render.options.cell_width = 4;
        render.options.cell_height = 3;

        let grid = state.entries.map_ref(RenderSquareSolution);
        let grid_widget = GridWidget::new(&grid, &cell_state);

        grid_widget.render(root, buf, &mut render);
    }

    fn on_command(
        &mut self,
        command: AppCommand<CrosswordApp>,
        resolver: AppResolver<CrosswordApp>,
        _ctx: &mut AppContext<CrosswordApp>,
    ) -> bool {
        match command {
            Command::Action {
                action: Action::Quit,
                ..
            } => resolver.quit(),
            _ => {}
        }

        true
    }
}
