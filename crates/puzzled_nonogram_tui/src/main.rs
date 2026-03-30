#![allow(dead_code)]
mod cli;
mod commands;
mod screens;

use std::{
    fs::File,
    io::{self, BufReader},
};

pub use cli::*;
pub use commands::*;
pub use screens::*;

// use clap::Parser;
use puzzled_io::puzzle_dir;
use puzzled_nonogram::{Nonogram, NonogramState};
use puzzled_tui::{App, GridRenderState, SidedGridRenderState, SidesRenderState, init_logging};

#[tokio::main]
async fn main() -> io::Result<()> {
    // let args = Args::parse();
    init_logging(true);

    let path = puzzle_dir::<Nonogram>()?.join("ladybug.json");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let puzzle: Nonogram = serde_json::from_reader(reader).map_err(io::Error::other)?;

    // let puzzle = Nonogram::load_text("tower").map_err(io::Error::other)?;
    let solve_state = NonogramState::from(&puzzle);

    let state = ();
    let mut app = App::<NonogramApp>::new(state)?;

    let render_state = GridRenderState {
        options: app.context.options.grid,
        rows: puzzle.fills().rows(),
        cols: puzzle.fills().cols(),
        ..Default::default()
    };
    let sided_render_state = SidedGridRenderState {
        grid: render_state,
        sides: SidesRenderState::default(),
    };

    let screen = PuzzleScreen::new(puzzle, solve_state, sided_render_state);

    app.run(Box::new(screen)).await?;

    Ok(())
}
