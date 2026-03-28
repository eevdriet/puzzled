#![allow(dead_code)]
mod cli;
mod commands;
mod screens;

use std::io;

pub use cli::*;
pub use commands::*;
pub use screens::*;

use clap::Parser;
use puzzled_io::TxtPuzzle;
use puzzled_nonogram::{Nonogram, NonogramState};
use puzzled_tui::{App, GridRenderState, init_logging};

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();
    init_logging(args.debug);

    let puzzle = Nonogram::load_text("tower").map_err(io::Error::other)?;
    let solve_state = NonogramState::from(&puzzle);

    let state = ();
    let mut app = App::<NonogramApp>::new(state)?;

    let render_state = GridRenderState {
        options: app.context.options.grid,
        rows: puzzle.fills().rows(),
        cols: puzzle.fills().cols(),
        ..Default::default()
    };

    let screen = PuzzleScreen::new(puzzle, solve_state, render_state);

    app.run(Box::new(screen)).await?;

    Ok(())
}
