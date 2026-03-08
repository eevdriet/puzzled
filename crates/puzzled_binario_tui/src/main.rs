mod action;
mod screens;
mod state;

pub use action::*;
pub use screens::*;
pub use state::*;

use ratatui::{
    layout::{HorizontalAlignment, VerticalAlignment},
    style::Style,
};
use std::io;

use puzzled_binario::Binario;
use puzzled_io::TxtPuzzle;
use puzzled_tui::{App, EventTrie, GridRenderState, init_logging};

#[tokio::main]
async fn main() -> io::Result<()> {
    init_logging(true);

    let (puzzle, solve_state) = Binario::load_text("special").map_err(io::Error::other)?;

    let mut render_state = GridRenderState::default();
    let opts = &mut render_state.options;

    opts.cell_width = 5;
    opts.cell_height = 3;

    let screen = PuzzleScreen::new(puzzle, solve_state, render_state);
    let events: EventTrie<BinarioAction> = EventTrie::from_config::<Binario>()?;

    // let action_keys = events.action_keys();
    // dbg!(action_keys);

    let state = AppState {};
    let mut app = App::new(state, events);

    app.run(Box::new(screen)).await?;

    Ok(())
}
