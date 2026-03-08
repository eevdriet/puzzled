mod action;
mod screens;
mod state;

pub use action::*;
pub use screens::*;
pub use state::*;

use std::io;

use puzzled_core::{Direction, Position};
use puzzled_crossword::Crossword;
use puzzled_io::TxtPuzzle;
use puzzled_tui::{App, EventTrie, GridOptions, GridRenderState, Viewport, init_logging};
use ratatui::{
    layout::{HorizontalAlignment, VerticalAlignment},
    style::Style,
};

#[tokio::main]
async fn main() -> io::Result<()> {
    init_logging(true);

    let (puzzle, solve_state) = Crossword::load_text("2026-03-07-nyt").map_err(io::Error::other)?;

    let render_state = GridRenderState {
        options: GridOptions {
            cell_width: 5,
            cell_height: 3,
            inner: None,
            // inner: Some(Size::new(5, 5)),
            inner_border_style: Style::default(),
            outer_border_style: Style::default(),
            draw_inner_borders: true,
            draw_outer_borders: false,
            h_align: HorizontalAlignment::Center,
            v_align: VerticalAlignment::Top,
        },
        viewport: Viewport::from_grid(puzzle.squares()),
        cursor: Position::default(),
        direction: Direction::default(),
    };

    let screen = PuzzleScreen::new(puzzle, solve_state, render_state);
    let events: EventTrie<CrosswordAction> = EventTrie::from_config::<Crossword>()?;

    let action_keys = events.action_keys();
    dbg!(action_keys);

    let state = AppState {};
    let mut app = App::new(state, events);

    app.run(Box::new(screen)).await?;

    Ok(())
}
