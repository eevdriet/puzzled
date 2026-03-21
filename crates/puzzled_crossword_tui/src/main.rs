mod commands;
mod screens;
mod state;

pub use commands::*;
pub use screens::*;
pub use state::*;

use std::io;

use puzzled_crossword::Crossword;
use puzzled_tui::{App, AppContext, EventTrie, init_logging};

#[tokio::main]
async fn main() -> io::Result<()> {
    init_logging(false);

    let events: EventTrie<CrosswordApp> = EventTrie::from_config::<Crossword>()?;
    let keys = events.action_keys();

    let state = AppState::default();
    let ctx = AppContext::new(state, keys);

    let mut app = App::new(ctx, events);

    let screen = TitleScreen::default();
    app.run(Box::new(screen)).await?;

    Ok(())
}
