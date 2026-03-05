pub mod read;
pub mod write;

use std::{fmt::Display, fs, io};

use puzzled_core::Puzzle;
pub use read::TxtReader;

use crate::puzzle_dir;

pub trait TxtPuzzle<S>: Puzzle + Display {
    fn read_text(input: &str) -> read::Result<(Self, S)>;
    fn write_text(&self, state: &S) -> String;

    fn load_text(name: &str) -> read::Result<(Self, S)> {
        let reader = TxtReader::new(false);

        let dir = puzzle_dir::<Self>()?;
        let path = dir.join(name).with_extension("txt");

        reader.read_from_path(path)
    }

    fn save_text(&self, name: &str) -> io::Result<()>
    where
        S: for<'a> From<&'a Self>,
    {
        let state = S::from(self);
        self.save_text_with_state(name, &state)
    }

    fn save_text_with_state(&self, name: &str, state: &S) -> io::Result<()> {
        let dir = puzzle_dir::<Self>()?;
        let path = dir.join(name).with_extension("txt");
        let text = self.write_text(state);

        fs::write(path, text)
    }
}
