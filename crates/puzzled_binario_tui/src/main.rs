mod action;
mod screens;
mod state;

pub use action::*;
use ratatui::{
    layout::{HorizontalAlignment, Size, VerticalAlignment},
    style::Style,
};
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

    let render_state = GridRenderState {
        options: GridOptions {
            cell_width: 5,
            cell_height: 3,
            inner: None,
            // inner: Some(Size::new(1, 1)),
            inner_border_style: Style::default(),
            outer_border_style: Style::default(),
            draw_inner_borders: false,
            draw_outer_borders: false,
            h_align: HorizontalAlignment::Center,
            v_align: VerticalAlignment::Top,
        },
        viewport: Viewport::from_grid(puzzle.cells()),
        cursor: Position::default(),
    };

    let screen = PuzzleScreen::new(puzzle, solve_state, render_state);
    let events: EventTrie<BinarioAction> = EventTrie::from_config::<Binario>()?;

    // let action_keys = events.action_keys();
    // dbg!(action_keys);

    let state = AppState {};
    let mut app = App::new(state, events);

    app.run(Box::new(screen)).await?;

    Ok(())
}
