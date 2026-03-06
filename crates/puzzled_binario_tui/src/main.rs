mod action;
mod screens;
mod state;

pub use action::*;
use ratatui::layout::Size;
pub use screens::*;
pub use state::*;

use std::io;

use puzzled_binario::Binario;
use puzzled_core::Position;
use puzzled_io::TxtPuzzle;
use puzzled_tui::{App, EventTrie, GridOptions, GridRenderState, Viewport, init_logging};

#[tokio::main]
async fn main() -> io::Result<()> {
    init_logging(true);

    let (puzzle, solve_state) = Binario::load_text("special").map_err(io::Error::other)?;
    let mut render_state = GridRenderState {
        options: GridOptions::default(),
        viewport: Viewport::from_grid(puzzle.cells()),
        cursor: Position::default(),
    };

    render_state.options.cell_width = 3;
    render_state.options.cell_height = 1;
    render_state.options.draw_inner_borders = false;

    let screen = PuzzleScreen::new(puzzle, solve_state, render_state);
    let events = EventTrie::from_config::<Binario>()?;

    let state = AppState {};
    let mut app = App::new(state, events);

    app.run(Box::new(screen)).await?;

    Ok(())
}
