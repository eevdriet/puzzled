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

    let (puzzle, solve_state) = Crossword::load_text("2026-03-07-nyt").map_err(io::Error::other)?;

    let mut render_state = GridRenderState::default();
    let opts = &mut render_state.options;

    opts.cell_width = 5;
    opts.cell_height = 3;
    // opts.inner = Some(ratatui::layout::Size::new(5, 5));
    // opts.draw_inner_borders = true;

    let screen = PuzzleScreen::new(puzzle, solve_state, render_state);
    let events: EventTrie<CrosswordMotion, CrosswordAction> =
        EventTrie::from_config::<Crossword>()?;

    let state = AppState::default();
    let ctx = AppContext::new(state);

    let mut app = App::new(ctx, events);

    app.run(Box::new(screen)).await?;

    Ok(())
}
