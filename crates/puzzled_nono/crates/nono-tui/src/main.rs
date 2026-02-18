#![allow(dead_code)]
mod actions;
mod app;
mod args;
mod config;
mod error;
mod events;
mod log;
mod widgets;

pub use actions::*;
pub use app::*;
pub use args::*;
pub use config::*;
pub use error::*;
pub use events::*;
pub use log::*;
pub use widgets::*;

use std::path::Path;

use clap::Parser;

fn main() -> Result<()> {
    let args = Args::parse();
    init_logging(&args)?;

    tracing::info!("Starting app");

    let path = Path::new("config.toml");
    if !path.exists() {
        return Err(Error::Custom("Couldn't config file".to_string()));
    }

    let contents = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&contents)
        .map_err(|err| Error::Custom(format!("Couldn't parse config file: {err}")))?;

    let nonogram = args.parse_puzzle()?;

    let puzzle = nonogram.puzzle;
    let rules = nonogram.rules;

    let style = PuzzleStyle {
        colors: nonogram.colors,
        grid_size: config.styles.grid_size,
        ..Default::default()
    };

    let mut term = ratatui::init();
    let mut app = App::new(puzzle, rules, style, config);

    if let Err(err) = app.run(&mut term) {
        tracing::error!("{err:#?}");
    }
    ratatui::restore();

    Ok(())
}
