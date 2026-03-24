mod commands;
mod screens;
mod state;

pub use commands::*;
pub use screens::*;
pub use state::*;

use std::io;

use puzzled_binario::{Binario, BinarioState};
use puzzled_io::TxtPuzzle;
use puzzled_tui::{App, GridRenderState, init_logging};

#[tokio::main]
async fn main() -> io::Result<()> {
    init_logging(true);

    let puzzle = Binario::load_text("special").map_err(io::Error::other)?;
    let solve_state = BinarioState::from(&puzzle);

    let mut render_state = GridRenderState::default();
    let opts = &mut render_state.options;

    opts.cell_width = 5;
    opts.cell_height = 3;

    let state = AppState {};
    let mut app = App::<BinarioApp>::new(state)?;

    let screen = PuzzleScreen::new(puzzle, solve_state, render_state);
    app.run(Box::new(screen)).await?;

    Ok(())
}
