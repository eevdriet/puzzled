mod builder;
mod clues;

pub use builder::*;

pub struct Puzzle {
    version: String,
    width: u8,
    height: u8,
    clue_count: u16,
}

impl Puzzle {
    pub fn new(version: String, width: u8, height: u8, clue_count: u16) -> Self {
        Self {
            version,
            width,
            height,
            clue_count,
        }
    }
}
