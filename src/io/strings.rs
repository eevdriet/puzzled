use crate::{
    Puzzle,
    io::{PuzRead, write::PuzWrite},
};

#[derive(Debug, Default)]
pub(crate) struct Strings {
    pub title: Vec<u8>,
    pub author: Vec<u8>,
    pub copyright: Vec<u8>,
    pub notes: Vec<u8>,
    pub clues: Vec<Vec<u8>>,
}

impl Strings {
    pub(crate) fn from_reader<R: PuzRead>(
        reader: &mut R,
        clue_count: usize,
    ) -> std::io::Result<Self> {
        let title = reader.read_str0()?;
        let author = reader.read_str0()?;
        let copyright = reader.read_str0()?;

        // Sequentially parse the clues
        let mut clues = Vec::with_capacity(clue_count);

        for num in 1..=clue_count {
            let context = format!("Clue #{num}");
            let clue = reader.read_str0()?;

            clues.push(clue);
        }

        let notes = reader.read_str0()?;

        Ok(Strings {
            title,
            author,
            copyright,
            notes,
            clues,
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
