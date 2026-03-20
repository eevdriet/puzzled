mod commands;
mod screens;
mod state;

pub use commands::*;
pub use screens::*;
pub use state::*;

use std::io;

use puzzled_crossword::Crossword;
use puzzled_io::TxtPuzzle;
use puzzled_tui::{App, AppContext, EventTrie, GridRenderState, init_logging};

#[tokio::main]
async fn main() -> io::Result<()> {
    init_logging(false);

    let events: EventTrie<CrosswordAction, CrosswordTextObject, CrosswordMotion> =
        EventTrie::from_config::<Crossword>()?;
    let keys = events.action_keys();

    let state = AppState::default();
    let ctx = AppContext::new(state, keys);

    let mut app = App::new(ctx, events);

    let screen = create_puzzle_screen()?;
    app.run(Box::new(screen)).await?;

    Ok(())
}

fn create_puzzle_screen() -> io::Result<PuzzleScreen> {
    let (puzzle, solve_state) = Crossword::load_text("2026-03-08-nyt").map_err(io::Error::other)?;

    let mut render_state = GridRenderState::default();
    let opts = &mut render_state.options;

    opts.cell_width = 5;
    opts.cell_height = 3;

    let events: EventTrie<CrosswordAction, CrosswordTextObject, CrosswordMotion> =
        EventTrie::from_config::<Crossword>()?;
    let keys = events.action_keys();
    let screen = PuzzleScreen::new(puzzle, solve_state, render_state, keys.clone());

    Ok(screen)
}
