use std::io::Read;

use crate::{
    Puzzle,
    io::{Span, write::PuzWrite},
};

#[derive(Debug, Default)]
pub(crate) struct Strings {
    pub title: Vec<u8>,
    pub author: Vec<u8>,
    pub copyright: Vec<u8>,
    pub notes: Vec<u8>,
    pub clues: Vec<Vec<u8>>,

    pub clues_span: Span,
}

impl Strings {
    pub(crate) fn from_reader<R: Read>(
        reader: &mut R,
        clue_count: usize,
        state: &mut PuzState<'a>,
    ) -> std::io::Result<Self> {
        let title = Vec::from(state.read_str("Title")?);
        let author = Vec::from(state.read_str("Author")?);
        let copyright = Vec::from(state.read_str("copyright")?);

        // Sequentially parse the clues
        let (clues, clues_span) = state.read_span(|s| {
            let mut clues = Vec::with_capacity(clue_count);

            for num in 1..=clue_count {
                let context = format!("Clue #{num}");
                let clue = Vec::from(s.read_str(context)?);

                clues.push(clue);
            }

            Ok(clues)
        })?;

        let notes = Vec::from(state.read_str("Notes")?);

        Ok(Strings {
            title,
            author,
            copyright,
            notes,
            clues,
            clues_span,
        })
    }

    pub(crate) fn from_puzzle(puzzle: &Puzzle) -> std::io::Result<Self> {
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

    pub(crate) fn write_with<W: PuzWrite>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&self.title)?;
        writer.write_all(&self.author)?;
        writer.write_all(&self.copyright)?;

        for clue in &self.clues {
            writer.write_all(clue)?;
        }

        writer.write_all(&self.notes)?;

        Ok(())
    }
}
