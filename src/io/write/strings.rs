use crate::{
    Puzzle,
    io::{PuzWriter, write::PuzWrite},
};

impl PuzWriter {
    pub(crate) fn write_strings(&self, puzzle: &Puzzle) -> std::io::Result<Vec<u8>> {
        let mut strings = Vec::new();
        strings.write_opt_str0(puzzle.title(), 0)?;
        strings.write_opt_str0(puzzle.author(), 0)?;
        strings.write_opt_str0(puzzle.copyright(), 0)?;

        for clue in puzzle.iter_clues() {
            strings.write_str0(&clue.text)?;
        }

        strings.write_opt_str0(puzzle.notes(), 0)?;

        Ok(strings)
    }
}
