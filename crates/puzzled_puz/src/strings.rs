use crate::{Context, PuzRead, PuzWrite, read, write};

/// [Strings](https://gist.github.com/sliminality/dab21fa834eae0a70193c7cd69c356d5#strings-section) section
///
/// This section ncludes the "metadata" for the [puzzle](crate::Puz).
/// It also includes the text for every [clue](puzzled::crossword::Clue) that should be placed in the puzzle [squares](puzzled::crossword::Squares).
/// The pseudo-code of how to do so is given [here](https://gist.github.com/sliminality/dab21fa834eae0a70193c7cd69c356d5#clue-assignment) and the crate implementation is found in [`Crossword::place_clues`](crate::Crossword::place_clues).
///
/// Below is an example of the order in which the strings should be read:
/// | Component | Length | Type | Example |
/// |:------------|:-------------------|:---|:---|
/// | Title       | ? | str | Theme: .PUZ format |
/// | Author      | ? | str | J. Puz / W. Shortz |
/// | Copyright   | ? | str | (c) 2007 J. Puz    |
/// | Clue `#1`   | ? | str | Cued, in pool      |
/// | ...         | ... | ... | ... |
/// | Clue `#n`   | ? | str | Quiet              |
/// | Notes       | ? | str | http://mywebsite   |
#[derive(Debug, Default)]
pub struct Strings {
    /// Title of the puzzle
    pub title: Vec<u8>,

    /// Author of the puzzle
    pub author: Vec<u8>,

    /// Copyright of the puzzle
    pub copyright: Vec<u8>,

    /// Notes on the puzzle
    pub notes: Vec<u8>,

    /// Clues to be placed in the puzzle
    pub clues: Vec<Vec<u8>>,
}

/// # Read
impl Strings {
    pub(crate) fn read_from<R: PuzRead>(reader: &mut R, clue_count: u16) -> read::Result<Self> {
        let title = reader.read_str0().context("Title")?;
        let author = reader.read_str0().context("Author")?;
        let copyright = reader.read_str0().context("Copyright")?;

        // Sequentially parse the clues
        let mut clues = Vec::with_capacity(clue_count as usize);

        for num in 1..=clue_count {
            let context = format!("Clue #{num}");
            let clue = reader.read_str0().context(context)?;

            clues.push(clue);
        }

        let notes = reader.read_str0().context("Notes")?;

        Ok(Strings {
            title,
            author,
            copyright,
            notes,
            clues,
        })
    }
}

/// # Write
impl Strings {
    pub(crate) fn write_with<W: PuzWrite>(&self, writer: &mut W) -> write::Result<()> {
        writer.write_all(&self.title).context("Title")?;
        writer.write_all(&self.author).context("Author")?;
        writer.write_all(&self.copyright).context("Copyright")?;

        for (idx, clue) in self.clues.iter().enumerate() {
            let num = idx + 1;
            let context = format!("Clue #{num}");
            writer.write_all(clue).context(context)?;
        }

        writer.write_all(&self.notes).context("Notes")?;

        Ok(())
    }
}
