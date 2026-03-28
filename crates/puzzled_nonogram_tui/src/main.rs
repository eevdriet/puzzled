#![allow(dead_code)]
// mod actions;
mod cli;
// mod config;
// mod error;
// mod events;
// mod widgets;

use std::io;

// pub use actions::*;
pub use cli::*;
// pub use config::*;
// pub use error::*;
// pub use events::*;
// pub use widgets::*;

use clap::Parser;
use puzzled_io::TxtPuzzle;
use puzzled_nonogram::Nonogram;
use puzzled_tui::init_logging;

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();
    init_logging(args.debug);

    let _puzzle = Nonogram::load_text("tower").map_err(io::Error::other)?;

    // if let Err(err) = app.run(&mut term) {
    //     tracing::error!("{err:#?}");
    // }
    // ratatui::restore();

    Ok(())
}
