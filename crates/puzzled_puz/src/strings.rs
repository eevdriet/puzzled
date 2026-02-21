use crate::{Context, PuzRead, PuzWrite, read, write};

#[doc(hidden)]
#[derive(Debug, Default)]
pub struct Strings {
    pub title: Vec<u8>,
    pub author: Vec<u8>,
    pub copyright: Vec<u8>,
    pub notes: Vec<u8>,
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
