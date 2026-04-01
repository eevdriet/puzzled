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

use puzzled_io::puzzle_dir;
use puzzled_nonogram::{Nonogram, NonogramState};
use puzzled_tui::{App, GridRenderState, SidedGridRenderState, SidesRenderState, init_logging};

#[tokio::main]
async fn main() -> io::Result<()> {
    init_logging(true);

    let path = puzzle_dir::<Nonogram>()?.join("ladybug.json");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let puzzle: Nonogram = serde_json::from_reader(reader).map_err(io::Error::other)?;

    // let puzzle = Nonogram::load_text("tower").map_err(io::Error::other)?;
    let solve_state = NonogramState::from(&puzzle);

    let state = ();
    let mut app = App::<NonogramApp>::new(state)?;

    // Set the cell width to be at least as wide as the largest number
    let mut options = app.context.options.grid;
    let max_count = puzzle
        .rules()
        .values()
        .map(|rule| {
            rule.runs()
                .iter()
                .map(|run| run.count)
                .max()
                .unwrap_or_default()
        })
        .max()
        .unwrap_or_default();
    let max_count_width = max_count.to_string().len() as u16;

    options.cell_width = max_count_width.max(options.cell_width);

    let render_state = GridRenderState {
        options,
        rows: puzzle.fills().rows(),
        cols: puzzle.fills().cols(),
        ..Default::default()
    };

    let sided_render_state = SidedGridRenderState::new(render_state, SidesRenderState::default());

    let screen = PuzzleScreen::new(puzzle, solve_state, sided_render_state);

    app.run(Box::new(screen)).await?;

    Ok(())
}
