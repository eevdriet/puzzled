mod commands;
mod screens;
mod state;

pub use commands::*;
pub use screens::*;
pub use state::*;

use std::io;

use puzzled_tui::{App, init_logging};

#[tokio::main]
async fn main() -> io::Result<()> {
    init_logging(false);

    let state = AppState::default();
    let mut app = App::<CrosswordApp>::new(state)?;

    let screen = TitleScreen::default();
    app.run(Box::new(screen)).await?;

    Ok(())
}
