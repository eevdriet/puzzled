use std::path::PathBuf;

use crate::PuzzleStyle;
use clap::Parser;
use puzzled_nono::{Nonogram, load_nonogram};

use crate::Result;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    pub file: PathBuf,

    #[arg(short = 'x', long, default_value_t = 'X')]
    pub fill_char: char,

    #[arg(short = 'c', long, default_value_t = 'o')]
    pub cross_char: char,

    #[arg(short, long, default_value_t = '.')]
    pub blank: char,

    #[arg(short, long)]
    pub debug: bool,
}

impl Args {
    pub fn parse_style(&self) -> PuzzleStyle {
        PuzzleStyle::default()
    }

    pub fn parse_puzzle(&self) -> Result<Nonogram> {
        let nonogram = load_nonogram(&self.file)?;
        Ok(nonogram)
    }
}
