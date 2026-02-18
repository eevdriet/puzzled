use crate::io::{Context, PuzWrite, Strings, write};

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
