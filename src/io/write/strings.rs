use crate::{
    Puzzle,
    io::{PuzWriter, Strings, write::PuzWrite},
};

impl PuzWriter {
    pub(crate) fn write_strings(&self, puzzle: &Puzzle) -> std::io::Result<Strings> {
        let mut strings = Strings::default();
        strings.title.write_opt_str0(puzzle.title(), 0)?;
        strings.author.write_opt_str0(puzzle.author(), 0)?;
        strings.copyright.write_opt_str0(puzzle.copyright(), 0)?;

        strings.clues = Vec::with_capacity(puzzle.clues().len());

        for (idx, clue) in puzzle.iter_clues().enumerate() {
            strings.clues[idx].write_str0(&clue.text)?;
        }

        strings.notes.write_opt_str0(puzzle.notes(), 0)?;

        Ok(strings)
    }
}
