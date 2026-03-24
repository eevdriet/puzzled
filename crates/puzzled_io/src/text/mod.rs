pub mod read;
pub mod write;

use std::{fmt::Display, fs, io};

use puzzled_core::Puzzle;
pub use read::TxtReader;

use crate::puzzle_dir;

pub trait TxtPuzzle: Puzzle + Display {
    fn read_text(input: &str) -> read::Result<Self>;
    fn write_text(&self) -> String;

    fn load_text(name: &str) -> read::Result<Self> {
        let dir = puzzle_dir::<Self>()?;
        let path = dir.join(name).with_extension("txt");

        let file_str = fs::read_to_string(path)?;
        Self::read_text(&file_str)
    }

    fn save_text(&self, name: &str) -> io::Result<()> {
        let dir = puzzle_dir::<Self>()?;
        let path = dir.join(name).with_extension("txt");

        fs::write(path, self.write_text())
    }
}
