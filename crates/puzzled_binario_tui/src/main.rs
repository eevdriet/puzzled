mod commands;
mod screens;
mod state;

pub use commands::*;
pub use screens::*;
pub use state::*;

use std::io;

use puzzled_binario::{Binario, BinarioState};
use puzzled_io::TxtPuzzle;
use puzzled_tui::{App, GridRenderState, SidedGridRenderState, SidesRenderState, init_logging};

#[tokio::main]
async fn main() -> io::Result<()> {
    init_logging(true);

    let puzzle = Binario::load_text("special").map_err(io::Error::other)?;
    let solve_state = BinarioState::from(&puzzle);

    let state = AppState {};
    let mut app = App::<BinarioApp>::new(state)?;

    let render_state = GridRenderState {
        options: app.context.options.grid,
        rows: puzzle.cells().rows(),
        cols: puzzle.cells().cols(),
        ..Default::default()
    };

    let render_state = SidedGridRenderState {
        grid: render_state,
        sides: SidesRenderState::default(),
    };

    let screen = PuzzleScreen::new(puzzle, solve_state, render_state);

    app.run(Box::new(screen)).await?;

    Ok(())
}
