#![allow(dead_code)]
mod actions;
mod cli;
mod config;
mod error;
mod events;
mod widgets;

pub use actions::*;
pub use cli::*;
pub use config::*;
pub use error::*;
pub use events::*;
use puzzled_tui::init_logging;
pub use widgets::*;

use clap::Parser;

#[tokio::main]
fn main() -> Result<()> {
    let args = Args::parse();
    init_logging(args.debug);

    let puzzle = 

    if let Err(err) = app.run(&mut term) {
        tracing::error!("{err:#?}");
    }
    ratatui::restore();

    Ok(())
}
