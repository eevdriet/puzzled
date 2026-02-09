mod builder;
mod clues;
mod line;
mod position;
mod timer;

use std::collections::BTreeMap;

pub use builder::*;
pub use clues::*;
pub use line::*;
pub use position::*;
pub use timer::*;

pub struct Puzzle {
    version: String,
    width: u8,
    height: u8,
    clue_count: u16,
    clues: BTreeMap<ClueLine, Clue>,
}

impl Puzzle {
    pub fn new(
        version: String,
        width: u8,
        height: u8,
        clue_count: u16,
        clues: BTreeMap<ClueLine, Clue>,
    ) -> Self {
        Self {
            version,
            width,
            height,
            clue_count,
            clues,
        }
    }
}
