mod action;
mod screens;
mod state;

pub use action::*;
pub use screens::*;
pub use state::*;

use std::io;

use derive_more::{Deref, DerefMut};
use puzzled_binario::{Binario, Bit};
use puzzled_core::{Position, grid};
use puzzled_tui::{App, CellRender, EventTrie, GridOptions, GridState, Viewport, init_logging};
use ratatui::{
    layout::Size,
    style::{Modifier, Style},
    text::Text,
};

const T: Bit = Bit::One;
const F: Bit = Bit::Zero;

#[derive(Debug, Deref, DerefMut)]
pub struct RenderBit(pub Bit);

impl CellRender<GridState> for RenderBit {
    fn render_cell(&self, pos: Position, state: &GridState) -> Text<'_> {
        let symbol = match pos == state.cursor {
            true => 'E'.to_string(),
            false => self.to_string(),
        };

        let mut style = Style::default();
        if pos == state.cursor {
            style = style.add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK);
        }

        Text::from(symbol).style(style)
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    init_logging(true);

    let grid = grid!(
        [T, T, F, F, F, F],
        [F, F, F, F, F, F],
        [F, F, F, T, T, F],
        [F, F, F, T, T, F],
        [F, F, F, T, T, F],
        [F, F, F, F, F, F],
    )
    .map(RenderBit);

    let mut state = GridState {
        options: GridOptions::default(),
        viewport: Viewport::from_grid(&grid),
        cursor: Position::default(),
    };

    state.options.cell_width = 4;
    state.options.cell_height = 2;
    state.options.inner = Some(Size::new(1, 1));
    // state.options.inner = None;
    state.options.draw_inner_borders = true;

    let screen = PuzzleScreen::new(grid, state);
    let events = EventTrie::from_config::<Binario>()?;

    let state = AppState {};
    let mut app = App::new(state, events);

    app.run(Box::new(screen)).await?;

    Ok(())
}
