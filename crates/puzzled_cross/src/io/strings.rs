use crate::{Puzzle, format, io::write::PuzWrite};

#[derive(Debug, Default)]
pub(crate) struct Strings {
    pub title: Vec<u8>,
    pub author: Vec<u8>,
    pub copyright: Vec<u8>,
    pub notes: Vec<u8>,
    pub clues: Vec<Vec<u8>>,
}

impl Strings {
    pub(crate) fn from_puzzle(puzzle: &Puzzle) -> format::Result<Self> {
        let mut strings = Strings::default();

        strings
            .title
            .write_opt_str0(puzzle.title(), 0)
            .expect("Title");
        strings
            .author
            .write_opt_str0(puzzle.author(), 0)
            .expect("Author");
        strings
            .copyright
            .write_opt_str0(puzzle.copyright(), 0)
            .expect("Copyright");

        strings.clues = Vec::with_capacity(puzzle.clues().len());

        for (idx, clue) in puzzle.iter_clues().enumerate() {
            let num = idx + 1;
            let context = format!("Clue #{num}");
            strings.clues[idx].write_str0(clue.text()).expect(&context);
        }

        strings
            .notes
            .write_opt_str0(puzzle.notes(), 0)
            .expect("Notes");

        Ok(strings)
    }
}
